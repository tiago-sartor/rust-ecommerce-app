use crate::backend::templates::*;
use crate::emails::Mailer;
use crate::middlewares::csrf::CsrfToken;
use crate::models::admin::Admin;
use crate::server::handlers::backend::payloads::{ForgotPasswordPayload, LoginPayload, ResetPasswordPayload};
use crate::shared::layouts::blank_layout::blank_layout;
use crate::utils::{AppError, Context, helpers, password};
use axum::{
    extract::{Extension, Form, Path, Query, State},
    response::{IntoResponse, Redirect, Response},
};
use hypertext::Renderable;
use sqlx::PgPool;
use std::collections::HashMap;
use time::OffsetDateTime;
use tower_sessions::Session;
use validator::Validate;

/**
 * === GET ===> /admin/login
 */
pub async fn admin_login_get(
    session: Session,
    Extension(token): Extension<CsrfToken>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    if session.get::<i64>("admin_id").await?.is_some() {
        return Ok(Redirect::to("/admin/dashboard").into_response());
    }

    let mut ctx = Context::<LoginPayload, ()>::new();
    ctx.csrf_token = token;

    if params.get("password_reset_success").map(|v| v == "true").unwrap_or(false) {
        ctx.flash_msg.insert(
            "password_reset_success".to_string(),
            "Your password has been successfully updated. You can now login.".to_string(),
        );
    }

    let template = admin_login_template(&ctx);
    let html = blank_layout("Admin Login", template, &ctx, None);

    Ok(html.render().into_response())
}

/**
 * === POST ===> /admin/login
 */
pub async fn admin_login_post(
    session: Session,
    Extension(token): Extension<CsrfToken>,
    State(pool): State<PgPool>,
    Form(payload): Form<LoginPayload>,
) -> Result<impl IntoResponse, AppError> {
    match Admin::get_by_email(&payload.email, &pool).await? {
        Some(admin) if password::verify_password(&admin.password_hash, &payload.password) => {
            session.insert("admin_id", admin.id).await?;
            helpers::regenerate_session(&session).await?;

            Ok(Redirect::to("/admin/dashboard").into_response())
        }
        _ => {
            let mut ctx = Context::new();
            ctx.payload = Form(payload);
            ctx.csrf_token = token;
            ctx.errors.insert("login".to_string(), "Invalid email or password".to_string());

            let template = admin_login_template(&ctx);
            let html = blank_layout("Admin Login", template, &ctx, None);

            Ok(html.render().into_response())
        }
    }
}

/**
 * === GET ===> /admin/forgot-password
 */
pub async fn admin_forgot_password_get(Extension(token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.csrf_token = token;

    let template = admin_forgot_password_template(&ctx);
    let html = blank_layout("Forgot Password", template, &ctx, None);

    Ok(html.render().into_response())
}

/**
 * === POST ===> /admin/forgot-password
 */
pub async fn admin_forgot_password_post(
    Extension(token): Extension<CsrfToken>,
    State(pool): State<PgPool>,
    Form(payload): Form<ForgotPasswordPayload>,
) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::<ForgotPasswordPayload, ()>::new();
    ctx.payload = Form(payload);
    ctx.csrf_token = token;

    if let Err(e) = ctx.payload.validate() {
        for (field, errs) in e.field_errors() {
            for err in errs {
                if let Some(message) = &err.message {
                    ctx.errors.insert(field.to_string(), message.to_string());
                }
            }
        }
    } else {
        ctx.flash_msg.insert(
            "forgot_password_success".to_string(),
            "If an account with this email exists, a password reset link will be sent.".to_string(),
        );
    }

    let reset_token = helpers::generate_random_token(64);
    Admin::update_reset_token(&reset_token, &ctx.payload.email, &pool).await?;
    let reset_link = format!("/admin/reset-password/{reset_token}");
    if let Ok(mailer) = Mailer::new(&pool) {
        if let Err(e) = mailer.send_password_reset_email(&ctx.payload.email, &reset_link).await {
            tracing::error!("Failed to send password reset email: {:?}", e);
        }
    }

    let template = admin_forgot_password_template(&ctx);
    let html = blank_layout("Forgot Password", template, &ctx, None);

    Ok(html.render().into_response())
}

/**
 * === GET ===> /admin/reset-password/{token}
 */
pub async fn admin_reset_password_get(
    Path(reset_token): Path<String>,
    Extension(csrf_token): Extension<CsrfToken>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let admin = Admin::get_by_reset_token(&reset_token, &pool).await?;

    if admin.filter(|a| a.reset_expires_at > Some(OffsetDateTime::now_utc())).is_none() {
        return Ok(Redirect::to("/admin/login").into_response());
    }

    let mut ctx = Context::new();
    ctx.csrf_token = csrf_token;

    let template = admin_reset_password_template(&ctx);
    let html = blank_layout("Reset Password", template, &ctx, None);

    Ok(html.render().into_response())
}

/**
 * === POST ===> /admin/reset-password/{token}
 */
pub async fn admin_reset_password_post(
    session: Session,
    Path(reset_token): Path<String>,
    Extension(csrf_token): Extension<CsrfToken>,
    State(pool): State<PgPool>,
    Form(payload): Form<ResetPasswordPayload>,
) -> Result<Response, AppError> {
    let admin = Admin::get_by_reset_token(&reset_token, &pool).await?;

    let Some(admin) = admin.filter(|a| a.reset_expires_at > Some(OffsetDateTime::now_utc())) else {
        return Ok(Redirect::to("/admin/login").into_response());
    };

    let mut ctx = Context::<ResetPasswordPayload, ()>::new();
    ctx.payload = Form(payload);
    ctx.csrf_token = csrf_token;

    if let Err(e) = ctx.payload.validate() {
        let errors = e
            .field_errors()
            .into_iter()
            .filter_map(|(field, errs)| Some((field.to_string(), errs.first()?.message.as_ref()?.to_string())))
            .collect();
        ctx.errors = errors;

        let template = admin_reset_password_template(&ctx);
        let html = blank_layout("Reset Password", template, &ctx, None);

        return Ok(html.render().into_response());
    }

    let hashed_password = password::hash_password(&ctx.payload.password)?;
    if Admin::update_password(&admin.id, &hashed_password, &pool).await.is_ok() {
        let _ = Admin::clear_reset_token(&admin.id, &pool).await;
        let _ = helpers::regenerate_session(&session).await;
        Ok(Redirect::to("/admin/login?password_reset_success=true").into_response())
    } else {
        ctx.errors
            .insert("internal_error".to_string(), "Failed to update password. Please try again.".to_string());

        let template = admin_reset_password_template(&ctx);
        let html = blank_layout("Reset Password", template, &ctx, None);

        Ok(html.render().into_response())
    }
}

/**
 * Logout
 */
pub async fn admin_logout(session: Session) -> Result<impl IntoResponse, AppError> {
    session.clear().await;
    helpers::regenerate_session(&session).await?;

    Ok(Redirect::to("/admin/login").into_response())
}
