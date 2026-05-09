use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use tower_sessions::Session;

pub async fn admin_auth(session: Session, request: Request, next: Next) -> Result<Response, StatusCode> {
    let admin_id: Option<i64> = session.get("admin_id").await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if admin_id.is_some() {
        Ok(next.run(request).await)
    } else {
        Ok(Redirect::to("/admin/login").into_response())
    }
}

pub async fn customer_auth(session: Session, request: Request, next: Next) -> Result<Response, StatusCode> {
    let customer_id: Option<i64> = session.get("customer_id").await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if customer_id.is_some() {
        Ok(next.run(request).await)
    } else {
        Ok(Redirect::to("/login").into_response())
    }
}
