use crate::backend::layouts::admin_layout::admin_layout;
use crate::backend::templates::admin_account::admin_account as admin_account_template;
use crate::backend::templates::admin_dashboard::admin_dashboard as admin_dashboard_template;
use crate::backend::templates::admin_forgot_password::admin_forgot_password as admin_forgot_password_template;
use crate::backend::templates::admin_login::admin_login as admin_login_template;
use crate::backend::templates::admin_reset_password::admin_reset_password as admin_reset_password_template;
use crate::models::admin::Admin;
use crate::shared::layouts::blank_layout::blank_layout;
use crate::utils::errors::AppError;
use crate::utils::helpers;
use axum::{
    extract::{Form, Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use hypertext::Renderable;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use time::OffsetDateTime;
use tower_sessions::Session;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct ForgotPasswordPayload {
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,
}

#[derive(Serialize, Deserialize, Validate)]
pub struct ResetPasswordPayload {
    #[validate(length(min = 8, max = 32, message = "Password must be between 8 and 32 characters"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Password does not match"))]
    pub confirm_password: String,
}

pub enum Type {
    Text(String),
    Number(i64),
    List(Vec<String>),
    Map(HashMap<String, String>),
    Bool(bool),
}

/// Helper for preparing the context for the templates
async fn prepare_context(session: &Session) -> Result<HashMap<String, Type>, AppError> {
    let mut context = HashMap::new();
    let csrf_token = session.get("csrf_token").await?.unwrap_or_default();
    context.insert("csrf_token".to_string(), Type::Text(csrf_token));

    Ok(context)
}

/// Helper for rendering with errors and payload state
macro_rules! render_with_errors {
    ($context:expr, $title:expr, $payload:expr, $errors:expr, $template_fn:expr) => {{
        let mut ctx = $context;
        ctx.insert("errors".to_string(), Type::Map($errors));
        ctx.insert("payload".to_string(), Type::Map(helpers::struct_to_map($payload)));
        let template = $template_fn(&ctx);
        blank_layout($title, template, &ctx).render().into_response()
    }};
}

/**
 * === GET ===> /admin/login
 */
pub async fn admin_login_get(session: Session, Query(params): Query<HashMap<String, String>>) -> Result<impl IntoResponse, AppError> {
    let mut context: HashMap<String, Type> = HashMap::new();

    if params.get("password_reset_success").map(|v| v == "true").unwrap_or(false) {
        context.insert("password_reset_success".to_string(), Type::Bool(true));
    }

    let token = session.get("csrf_token").await?.unwrap_or_default();
    context.insert("csrf_token".to_string(), Type::Text(token));

    let template = admin_login_template(&context);
    Ok(blank_layout("Admin Login", template, &context).render().into_response())
}

/**
 * === POST ===> /admin/login
 */
pub async fn admin_login_post(State(pool): State<PgPool>, session: Session, Form(payload): Form<LoginPayload>) -> Result<impl IntoResponse, AppError> {
    match Admin::get_by_email(&pool, &payload.email).await? {
        Some(admin) if helpers::verify_password(&admin.password_hash, &payload.password) => {
            session.insert("admin_id", admin.id).await?;
            session.insert("csrf_token", helpers::generate_random_token(64)).await?;

            Ok(Redirect::to("/admin/dashboard").into_response())
        }
        _ => {
            let mut context: HashMap<String, Type> = HashMap::new();

            let payload_map = helpers::struct_to_map(&payload);
            context.insert("payload".to_string(), Type::Map(payload_map));

            let mut errors = HashMap::new();
            errors.insert("login".to_string(), "Invalid email or password".to_string());
            context.insert("errors".to_string(), Type::Map(errors));

            let token = session.get("csrf_token").await?.unwrap_or_default();
            context.insert("csrf_token".to_string(), Type::Text(token));

            let template = admin_login_template(&context);
            Ok(blank_layout("Admin Login", template, &context).render().into_response())
        }
    }
}

/**
 * === GET ===> /admin/forgot-password
 */
pub async fn admin_forgot_password_get(session: Session) -> Result<impl IntoResponse, AppError> {
    let mut context: HashMap<String, Type> = HashMap::new();

    let token = session.get("csrf_token").await?.unwrap_or_default();
    context.insert("csrf_token".to_string(), Type::Text(token));

    let template = admin_forgot_password_template(&context);
    Ok(blank_layout("Forgot Password", template, &context).render().into_response())
}

/**
 * === POST ===> /admin/forgot-password
 */
pub async fn admin_forgot_password_post(State(pool): State<PgPool>, session: Session, Form(payload): Form<ForgotPasswordPayload>) -> Result<impl IntoResponse, AppError> {
    let mut context: HashMap<String, Type> = HashMap::new();
    let mut errors = HashMap::new();

    // Verify if payload is valid
    if let Err(e) = payload.validate() {
        for (field, errs) in e.field_errors() {
            for err in errs {
                if let Some(ref message) = err.message {
                    errors.insert(field.to_string(), message.to_string());
                }
            }
        }
        context.insert("errors".to_string(), Type::Map(errors));
    } else {
        context.insert("forgot_password_success".to_string(), Type::Bool(true));
    }

    // Check if the email is associated to a user in the database
    let admin_exists = Admin::get_by_email(&pool, &payload.email).await?.is_some();
    if admin_exists {
        // Send an email with a password reset link here
    }

    let token = session.get("csrf_token").await?.unwrap_or_default();
    context.insert("csrf_token".to_string(), Type::Text(token));

    let payload_map = helpers::struct_to_map(&payload);
    context.insert("payload".to_string(), Type::Map(payload_map));

    let template = admin_forgot_password_template(&context);
    Ok(blank_layout("Forgot Password", template, &context).render().into_response())
}

/**
 * === GET ===> /admin/reset-password/{token}
 */
pub async fn admin_reset_password_get(Path(token): Path<String>, session: Session, State(pool): State<PgPool>) -> Result<impl IntoResponse, AppError> {
    let admin = Admin::get_by_reset_token(&pool, &token).await?;

    if admin.filter(|a| a.reset_expires_at > Some(OffsetDateTime::now_utc())).is_none() {
        return Ok(Redirect::to("/admin/login").into_response());
    }

    let mut context: HashMap<String, Type> = HashMap::new();

    let csrf_token = session.get("csrf_token").await?.unwrap_or_default();
    context.insert("csrf_token".to_string(), Type::Text(csrf_token));

    let template = admin_reset_password_template(&context);
    Ok(blank_layout("Reset Password", template, &context).render().into_response())
}

/**
 * === POST ===> /admin/reset-password/{token}
 */
pub async fn admin_reset_password_post(Path(token): Path<String>, session: Session, State(pool): State<PgPool>, Form(payload): Form<ResetPasswordPayload>) -> Result<Response, AppError> {
    // 1. Guard against invalid/expired token early
    let admin = Admin::get_by_reset_token(&pool, &token).await?;
    let Some(admin) = admin.filter(|a| a.reset_expires_at > Some(OffsetDateTime::now_utc())) else {
        return Ok(Redirect::to("/admin/login").into_response());
    };

    let context = prepare_context(&session).await?;
    let mut errors = HashMap::new();

    // 2. Validate payload
    if let Err(e) = payload.validate() {
        let errors = e
            .field_errors()
            .into_iter()
            .filter_map(|(field, errs)| Some((field.to_string(), errs.first()?.message.as_ref()?.to_string())))
            .collect();
        return Ok(render_with_errors!(context, "Reset Password", &payload, errors, admin_reset_password_template));
    }

    // 3. Update password
    let hashed_password = helpers::hash_password(&payload.password)?;
    if Admin::update_password(&pool, &admin.id, &hashed_password).await.is_ok() {
        let _ = Admin::clear_reset_token(&pool, &admin.id).await;
        let _ = helpers::regenerate_session(&session).await;
        Ok(Redirect::to("/admin/login?password_reset_success=true").into_response())
    } else {
        errors.insert("internal_error".to_string(), "Failed to update password. Please try again.".to_string());
        Ok(render_with_errors!(context, "Reset Password", &payload, errors, admin_reset_password_template))
    }
}

pub async fn admin_logout(session: Session) -> Result<impl IntoResponse, AppError> {
    session.clear().await;
    helpers::regenerate_session(&session).await?;

    Ok(Redirect::to("/admin/login").into_response())
}

pub async fn admin_dashboard(State(pool): State<PgPool>, session: Session) -> Result<impl IntoResponse, AppError> {
    let admin_id: Option<i64> = session.get("admin_id").await.unwrap_or_default();
    if let Some(id) = admin_id {
        if let Some(admin) = Admin::get_by_id(&pool, &id).await? {
            let mut context: HashMap<String, Type> = HashMap::new();

            let token = session.get("csrf_token").await?.unwrap_or_default();
            context.insert("csrf_token".to_string(), Type::Text(token));

            let admin_map = helpers::struct_to_map(&admin);
            context.insert("admin".to_string(), Type::Map(admin_map));

            let template = admin_dashboard_template(&context);
            return Ok(admin_layout("Dashboard", template, &context).render().into_response());
        }
    }

    Ok(Redirect::to("/admin/login").into_response())
}

pub async fn admin_account(State(pool): State<PgPool>, session: Session) -> Result<impl IntoResponse, AppError> {
    let admin_id: Option<i64> = session.get("admin_id").await.unwrap_or_default();
    if let Some(id) = admin_id {
        if let Some(admin) = Admin::get_by_id(&pool, &id).await? {
            let mut context: HashMap<String, Type> = HashMap::new();
            let token = session.get("csrf_token").await?.unwrap_or_default();
            context.insert("csrf_token".to_string(), Type::Text(token));

            let admin_map = helpers::struct_to_map(&admin);
            context.insert("admin".to_string(), Type::Map(admin_map));

            let template = admin_account_template(&context);
            return Ok(admin_layout("Account", template, &context).render().into_response());
        }
    }

    Ok(Redirect::to("/admin/login").into_response())
}
