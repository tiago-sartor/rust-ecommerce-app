use crate::models::admin::Admin;
use crate::models::customer::Customer;
use crate::utils::AppError;
use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use sqlx::PgPool;
use tower_sessions::Session;

pub async fn admin_auth(State(pool): State<PgPool>, session: Session, mut request: Request, next: Next) -> Result<Response, AppError> {
    if let Some(id) = session.get("admin_id").await? {
        if let Some(admin) = Admin::get_by_id(&id, &pool).await? {
            request.extensions_mut().insert(admin);

            return Ok(next.run(request).await);
        }
    }

    Ok(Redirect::to("/admin/login").into_response())
}

pub async fn customer_auth(State(pool): State<PgPool>, session: Session, mut request: Request, next: Next) -> Result<Response, AppError> {
    if let Some(id) = session.get("customer_id").await? {
        if let Some(customer) = Customer::get_by_id(&id, &pool).await? {
            request.extensions_mut().insert(customer);

            return Ok(next.run(request).await);
        }
    }

    Ok(Redirect::to("/login").into_response())
}
