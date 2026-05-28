use crate::backend::{layouts::*, templates::*};
use crate::middlewares::csrf::CsrfToken;
use crate::models::admin::Admin;
use crate::shared::layouts::blank_layout::blank_layout;
use crate::utils::{errors::AppError, helpers, password};
use crate::utils::context::Context;
use axum::{
    extract::{Form, Path, Query, State},
    response::{IntoResponse, Redirect, Response},
};
use hypertext::Renderable;
use sqlx::PgPool;
use std::collections::HashMap;
use tower_sessions::Session;
use validator::Validate;
use time::OffsetDateTime;
use crate::emails::Mailer;

#[derive(serde::Deserialize, validator::Validate, Default)]
pub struct LoginPayload {
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, validator::Validate, Default)]
pub struct ForgotPasswordPayload {
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,
}

#[derive(serde::Deserialize, validator::Validate, Default)]
pub struct ResetPasswordPayload {
    #[validate(length(min = 8, max = 32, message = "Password must be between 8 and 32 characters"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Password does not match"))]
    pub confirm_password: String,
}

/**
 * === GET ===> /admin/login
 */
pub async fn admin_login_get(
    session: Session,
    axum::extract::Extension(token): axum::extract::Extension<CsrfToken>,
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
    Ok(blank_layout("Admin Login", template, &ctx).render().into_response())
}

/**
 * === POST ===> /admin/login
 */
pub async fn admin_login_post(
    session: Session,
    axum::extract::Extension(token): axum::extract::Extension<CsrfToken>,
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
            Ok(blank_layout("Admin Login", template, &ctx).render().into_response())
        }
    }
}

/**
 * === GET ===> /admin/forgot-password
 */
pub async fn admin_forgot_password_get(axum::extract::Extension(token): axum::extract::Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.csrf_token = token;

    let template = admin_forgot_password_template(&ctx);
    Ok(blank_layout("Forgot Password", template, &ctx).render().into_response())
}

/**
 * === POST ===> /admin/forgot-password
 */
pub async fn admin_forgot_password_post(
    axum::extract::Extension(token): axum::extract::Extension<CsrfToken>,
    State(pool): State<PgPool>,
    Form(payload): Form<ForgotPasswordPayload>,
) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::<ForgotPasswordPayload, ()>::new();
    ctx.payload = Form(payload);
    ctx.csrf_token = token;

    if let Err(e) = ctx.payload.validate() {
        for (field, errs) in e.field_errors() {
            for err in errs {
                if let Some(ref message) = err.message {
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
    Ok(blank_layout("Forgot Password", template, &ctx).render().into_response())
}

/**
 * === GET ===> /admin/reset-password/{token}
 */
pub async fn admin_reset_password_get(
    Path(reset_token): Path<String>,
    axum::extract::Extension(csrf_token): axum::extract::Extension<CsrfToken>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let admin = Admin::get_by_reset_token(&reset_token, &pool).await?;

    if admin.filter(|a| a.reset_expires_at > Some(OffsetDateTime::now_utc())).is_none() {
        return Ok(Redirect::to("/admin/login").into_response());
    }

    let mut ctx = Context::new();
    ctx.csrf_token = csrf_token;

    let template = admin_reset_password_template(&ctx);
    Ok(blank_layout("Reset Password", template, &ctx).render().into_response())
}

/**
 * === POST ===> /admin/reset-password/{token}
 */
pub async fn admin_reset_password_post(
    session: Session,
    Path(reset_token): Path<String>,
    axum::extract::Extension(csrf_token): axum::extract::Extension<CsrfToken>,
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
        return Ok(blank_layout("Reset Password", template, &ctx).render().into_response());
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
        Ok(blank_layout("Reset Password", template, &ctx).render().into_response())
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