use std::collections::HashMap;

use crate::backend::layouts::admin_layout::admin_layout;
use crate::backend::templates::admin_account::admin_account as admin_account_template;
use crate::backend::templates::admin_dashboard::admin_dashboard as admin_dashboard_template;
use crate::backend::templates::admin_forgot_password::admin_forgot_password as admin_forgot_password_template;
use crate::backend::templates::admin_login::admin_login as admin_login_template;
use crate::backend::templates::admin_reset_password::admin_reset_password as admin_reset_password_template;
use crate::helpers;
use crate::models::admin::Admin;
use crate::shared::layouts::blank_layout::blank_layout;

use axum::{
    extract::{Form, Query, State},
    response::{IntoResponse, Redirect},
};
use hypertext::Renderable;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
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

/**
 * === GET ===> /admin/login
 */
pub async fn admin_login_get(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    let mut context: HashMap<String, Type> = HashMap::new();

    if params.get("password_reset_success").map(|v| v == "true").unwrap_or(false) {
        context.insert("password_reset_success".to_string(), Type::Bool(true));
    }

    let template = admin_login_template(&context);
    blank_layout("Admin Login", template, &context).render().into_response()
}

/**
 * === POST ===> /admin/login
 */
pub async fn admin_login_post(State(pool): State<PgPool>, session: Session, Form(payload): Form<LoginPayload>) -> impl IntoResponse {
    match Admin::get_by_email(&pool, &payload.email).await {
        Ok(Some(admin)) if admin.verify_password(&payload.password) => {
            session.insert("admin_id", admin.id).await.unwrap();
            Redirect::to("/admin/dashboard").into_response()
        }
        _ => {
            let mut context: HashMap<String, Type> = HashMap::new();
            context.insert("csrf_token".to_string(), Type::Text("abc123xyz456".to_string()));

            let payload_map = helpers::struct_to_map(&payload);
            context.insert("payload".to_string(), Type::Map(payload_map));

            let mut errors = HashMap::new();
            errors.insert("login".to_string(), "Invalid email or password".to_string());
            context.insert("errors".to_string(), Type::Map(errors));

            let template = admin_login_template(&context);
            blank_layout("Admin Login", template, &context).render().into_response()
        }
    }
}

/**
 * === GET ===> /admin/forgot-password
 */
pub async fn admin_forgot_password_get() -> impl IntoResponse {
    let context: HashMap<String, Type> = HashMap::new();
    let template = admin_forgot_password_template(&context);
    blank_layout("Forgot Password", template, &context).render().into_response()
}

/**
 * === POST ===> /admin/forgot-password
 */
pub async fn admin_forgot_password_post(State(pool): State<PgPool>, Form(payload): Form<ForgotPasswordPayload>) -> impl IntoResponse {
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
    let admin_exists = Admin::get_by_email(&pool, &payload.email).await.unwrap_or(None).is_some();
    if admin_exists {
        // Send an email with a password reset link here
    }

    context.insert("csrf_token".to_string(), Type::Text("abc123xyz456".to_string()));

    let payload_map = helpers::struct_to_map(&payload);
    context.insert("payload".to_string(), Type::Map(payload_map));

    let template = admin_forgot_password_template(&context);
    blank_layout("Forgot Password", template, &context).render().into_response()
}

/**
 * === GET ===> /admin/reset-password
 */
pub async fn admin_reset_password_get() -> impl IntoResponse {
    let context: HashMap<String, Type> = HashMap::new();
    let template = admin_reset_password_template(&context);
    blank_layout("Reset Password", template, &context).render().into_response()
}

/**
 * === POST ===> /admin/reset-password
 */
pub async fn admin_reset_password_post(Form(payload): Form<ResetPasswordPayload>) -> impl IntoResponse {
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

        let payload_map = helpers::struct_to_map(&payload);
        context.insert("payload".to_string(), Type::Map(payload_map));

        let template = admin_reset_password_template(&context);
        blank_layout("Reset Password", template, &context).render().into_response()
    } else {
        Redirect::to("/admin/login?password_reset_success=true").into_response()
    }
}

pub async fn admin_logout(session: Session) -> impl IntoResponse {
    session.clear().await;
    Redirect::to("/admin/login")
}

pub async fn admin_dashboard(State(pool): State<PgPool>, session: Session) -> impl IntoResponse {
    let admin_id: Option<i64> = session.get("admin_id").await.unwrap_or(None);
    if let Some(id) = admin_id {
        if let Ok(Some(admin)) = Admin::get_by_id(&pool, id).await {
            let mut context: HashMap<String, Type> = HashMap::new();
            context.insert("csrf_token".to_string(), Type::Text("abc123xyz456".to_string()));

            let admin_map = helpers::struct_to_map(&admin);
            context.insert("admin".to_string(), Type::Map(admin_map));

            let template = admin_dashboard_template(&context);
            return admin_layout("Dashboard", template, &context).render().into_response();
        }
    }

    Redirect::to("/admin/login").into_response()
}

pub async fn admin_account(State(pool): State<PgPool>, session: Session) -> impl IntoResponse {
    let admin_id: Option<i64> = session.get("admin_id").await.unwrap_or(None);
    if let Some(id) = admin_id {
        if let Ok(Some(admin)) = Admin::get_by_id(&pool, id).await {
            let mut context: HashMap<String, Type> = HashMap::new();
            context.insert("csrf_token".to_string(), Type::Text("abc123xyz456".to_string()));

            let admin_map = helpers::struct_to_map(&admin);
            context.insert("admin".to_string(), Type::Map(admin_map));

            let template = admin_account_template(&context);
            return admin_layout("Account", template, &context).render().into_response();
        }
    }

    Redirect::to("/admin/login").into_response()
}
