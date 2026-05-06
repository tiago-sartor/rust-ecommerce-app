use crate::backend::layouts::admin_layout::admin_layout;
use crate::backend::templates::admin_account::admin_account as admin_account_template;
use crate::backend::templates::admin_dashboard::admin_dashboard as admin_dashboard_template;
use crate::models::admin::Admin;
use axum::{
    extract::{Form, State},
    response::{IntoResponse, Redirect},
};
use hypertext::Renderable;
use serde::Deserialize;
use sqlx::PgPool;
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub async fn admin_login_get() -> impl IntoResponse {
    // For now, return a placeholder or render the template if available
    // admin_layout("Login", admin_login_template(), "").render()
    "Login Page (GET) - Implementation in progress".into_response()
}

pub async fn admin_login_post(
    State(pool): State<PgPool>,
    session: Session,
    Form(payload): Form<LoginPayload>,
) -> impl IntoResponse {
    match Admin::get_by_email(&pool, &payload.email).await {
        Ok(Some(admin)) if admin.verify_password(&payload.password) => {
            session.insert("admin_id", admin.id).await.unwrap();
            Redirect::to("/admin/dashboard").into_response()
        }
        _ => {
            // In a real app, you'd pass an error message to the template
            Redirect::to("/admin/login").into_response()
        }
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
            return admin_layout("Dashboard", admin_dashboard_template(&admin), "", &admin)
                .render()
                .into_response();
        }
    }

    Redirect::to("/admin/login").into_response()
}

pub async fn admin_account(State(pool): State<PgPool>, session: Session) -> impl IntoResponse {
    let admin_id: Option<i64> = session.get("admin_id").await.unwrap_or(None);

    if let Some(id) = admin_id {
        if let Ok(Some(admin)) = Admin::get_by_id(&pool, id).await {
            return admin_layout("Account", admin_account_template(&admin), "", &admin)
                .render()
                .into_response();
        }
    }

    Redirect::to("/admin/login").into_response()
}
