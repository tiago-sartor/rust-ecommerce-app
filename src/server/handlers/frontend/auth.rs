use crate::models::customer::Customer;
use crate::utils::{AppError, helpers};
use axum::{
    extract::{Form, State},
    response::{Html, IntoResponse, Redirect},
};
use serde::Deserialize;
use sqlx::PgPool;
use tower_sessions::Session;

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

pub async fn login_page() -> Html<String> {
    Html("<h1>Login Page</h1>".to_string())
}

pub async fn customer_login_post(
    State(pool): State<PgPool>,
    session: Session,
    Form(payload): Form<LoginPayload>,
) -> Result<impl IntoResponse, AppError> {
    match Customer::get_by_email(&payload.email, &pool).await? {
        Some(customer) if crate::utils::password::verify_password(&customer.password_hash, &payload.password) => {
            session.insert("customer_id", customer.id).await?;
            session.insert("csrf_token", helpers::generate_random_token(64)).await?;

            Ok(Redirect::to("/").into_response())
        }
        _ => Ok(Redirect::to("/login").into_response()),
    }
}

pub async fn customer_logout(session: Session) -> Result<impl IntoResponse, AppError> {
    session.clear().await;
    helpers::regenerate_session(&session).await?;

    Ok(Redirect::to("/login").into_response())
}
