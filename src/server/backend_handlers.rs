use crate::backend::layouts::admin_layout::admin_layout;
use crate::backend::templates::{
    admin_account::admin_account_template, admin_add_order::admin_add_order_template, admin_add_product::admin_add_product_template,
    admin_dashboard::admin_dashboard_template, admin_edit_product::admin_edit_product_template, admin_emails::admin_emails_template,
    admin_forgot_password::admin_forgot_password_template, admin_login::admin_login_template, admin_order_details::admin_order_details_template,
    admin_orders::admin_orders_template, admin_products::admin_products_template, admin_reset_password::admin_reset_password_template,
    admin_settings::admin_settings_template,
};
use crate::emails::mailer::{EmailLog, Mailer};
use crate::middlewares::csrf::CsrfToken;
use crate::models::admin::Admin;
use crate::shared::layouts::blank_layout::blank_layout;
use crate::utils::{errors::AppError, helpers, password};
use axum::{
    extract::{Extension, Form, Path, Query, State},
    response::{IntoResponse, Redirect, Response},
};
use hypertext::Renderable;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use time::OffsetDateTime;
use tower_sessions::Session;
use validator::Validate;

#[derive(Default, Serialize, Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,
    pub password: String,
}

#[derive(Default, Serialize, Deserialize, Validate)]
pub struct ForgotPasswordPayload {
    #[validate(email(message = "Please enter a valid email address"))]
    pub email: String,
}

#[derive(Default, Serialize, Deserialize, Validate)]
pub struct ResetPasswordPayload {
    #[validate(length(min = 8, max = 32, message = "Password must be between 8 and 32 characters"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Password does not match"))]
    pub confirm_password: String,
}

pub struct Context<P = (), D = ()> {
    pub admin: Admin,
    pub csrf_token: CsrfToken,
    pub payload: Form<P>,
    pub data: D,
    pub errors: HashMap<String, String>,
    pub flash_msg: HashMap<String, String>,
}

impl<P: Default, D: Default> Context<P, D> {
    pub fn new() -> Self {
        Context {
            admin: Admin::new(),
            csrf_token: CsrfToken(String::new()),
            payload: Form(P::default()),
            data: D::default(),
            errors: HashMap::new(),
            flash_msg: HashMap::new(),
        }
    }
}

/**
 * === GET ===> /admin/login
 */
pub async fn admin_login_get(
    session: Session,
    Extension(token): Extension<CsrfToken>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    // If the admin is already logged in, redirect to the dashboard
    if session.get::<i64>("admin_id").await?.is_some() {
        return Ok(Redirect::to("/admin/dashboard").into_response());
    }

    let mut ctx = Context::new();
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
    Extension(token): Extension<CsrfToken>,
    State(pool): State<PgPool>,
    Form(payload): Form<LoginPayload>,
) -> Result<impl IntoResponse, AppError> {
    match Admin::get_by_email(&pool, &payload.email).await? {
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
pub async fn admin_forgot_password_get(Extension(token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.csrf_token = token;

    let template = admin_forgot_password_template(&ctx);
    Ok(blank_layout("Forgot Password", template, &ctx).render().into_response())
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

    // Verify if payload is valid
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

    // // Check if the email is associated to a user in the database
    // let admin = Admin::get_by_email(&pool, &ctx.payload.email).await?;
    // if let Some(admin) = admin {
        // Generate a new reset token
        let reset_token = helpers::generate_random_token(64);
        Admin::update_reset_token(&pool, &reset_token, &ctx.payload.email).await?;
        // Send the reset email
        let reset_link = format!("/admin/reset-password/{reset_token}");
        if let Ok(mailer) = Mailer::new(&pool) {
            let _ = mailer.send_password_reset_email(&ctx.payload.email, &reset_link).await;
        }
    // }

    let template = admin_forgot_password_template(&ctx);
    Ok(blank_layout("Forgot Password", template, &ctx).render().into_response())
}

/**
 * === GET ===> /admin/reset-password/{token}
 */
pub async fn admin_reset_password_get(
    Path(reset_token): Path<String>,
    Extension(csrf_token): Extension<CsrfToken>,
    State(pool): State<PgPool>,
) -> Result<impl IntoResponse, AppError> {
    let admin = Admin::get_by_reset_token(&pool, &reset_token).await?;

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
    Extension(csrf_token): Extension<CsrfToken>,
    State(pool): State<PgPool>,
    Form(payload): Form<ResetPasswordPayload>,
) -> Result<Response, AppError> {
    // 1. Guard against invalid/expired token early
    let admin = Admin::get_by_reset_token(&pool, &reset_token).await?;

    let Some(admin) = admin.filter(|a| a.reset_expires_at > Some(OffsetDateTime::now_utc())) else {
        return Ok(Redirect::to("/admin/login").into_response());
    };

    let mut ctx = Context::<ResetPasswordPayload, ()>::new();
    ctx.payload = Form(payload);
    ctx.csrf_token = csrf_token;

    // 2. Validate payload
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

    // 3. Update password
    let hashed_password = password::hash_password(&ctx.payload.password)?;
    if Admin::update_password(&pool, &admin.id, &hashed_password).await.is_ok() {
        let _ = Admin::clear_reset_token(&pool, &admin.id).await;
        let _ = helpers::regenerate_session(&session).await;
        Ok(Redirect::to("/admin/login?password_reset_success=true").into_response())
    } else {
        ctx.errors
            .insert("internal_error".to_string(), "Failed to update password. Please try again.".to_string());

        let template = admin_reset_password_template(&ctx);
        Ok(blank_layout("Reset Password", template, &ctx).render().into_response())
    }
}

pub async fn admin_logout(session: Session) -> Result<impl IntoResponse, AppError> {
    session.clear().await;
    helpers::regenerate_session(&session).await?;

    Ok(Redirect::to("/admin/login").into_response())
}

pub async fn admin_dashboard(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_dashboard_template(&ctx);
    Ok(admin_layout("Dashboard", template, &ctx).render().into_response())
}

pub async fn admin_orders(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_orders_template(&ctx);
    Ok(admin_layout("Orders", template, &ctx).render().into_response())
}

pub async fn admin_add_order(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_add_order_template(&ctx);
    Ok(admin_layout("Add Order", template, &ctx).render().into_response())
}

pub async fn admin_order_details(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_order_details_template(&ctx);
    Ok(admin_layout("Order Details", template, &ctx).render().into_response())
}

pub async fn admin_products(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_products_template(&ctx);
    Ok(admin_layout("Products", template, &ctx).render().into_response())
}

pub async fn admin_add_product(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_add_product_template(&ctx);
    Ok(admin_layout("Add Product", template, &ctx).render().into_response())
}

pub async fn admin_edit_product(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_edit_product_template(&ctx);
    Ok(admin_layout("Edit Product", template, &ctx).render().into_response())
}

pub async fn admin_account(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_account_template(&ctx);
    Ok(admin_layout("Account", template, &ctx).render().into_response())
}

pub async fn admin_emails(
    State(pool): State<PgPool>,
    Extension(admin): Extension<Admin>,
    Extension(csrf_token): Extension<CsrfToken>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::<(), (Vec<EmailLog>, i64, i64, i64)>::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let page = params.get("page").and_then(|v| v.parse::<i64>().ok().filter(|&v| v > 0)).unwrap_or(1);
    let limit = params.get("limit").and_then(|v| v.parse::<i64>().ok().filter(|&v| v > 9)).unwrap_or(10);
    //let search = params.get("search").cloned().unwrap_or_default();

    let (logs, count) = Mailer::get_logs_paginated(&pool, page, limit).await.unwrap_or_default();
    ctx.data = (logs, count, page, limit);

    let template = admin_emails_template(&ctx);
    Ok(admin_layout("Email Logs", template, &ctx).render().into_response())
}

pub async fn admin_settings(Extension(admin): Extension<Admin>, Extension(csrf_token): Extension<CsrfToken>) -> Result<impl IntoResponse, AppError> {
    let mut ctx = Context::new();
    ctx.admin = admin;
    ctx.csrf_token = csrf_token;

    let template = admin_settings_template(&ctx);
    Ok(admin_layout("Settings", template, &ctx).render().into_response())
}
