use axum::{
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
};
use tower_sessions::Session;

pub async fn csrf_middleware(session: Session, req: Request, next: Next) -> Response {
    if matches!(req.method().as_str(), "POST" | "PUT" | "DELETE" | "PATCH") {
        let session_token: Option<String> = session.get("csrf_token").await.unwrap_or(None);
        let header_token = req.headers().get("X-CSRF-Token").and_then(|h| h.to_str().ok());

        if session_token.is_none() || header_token.is_none() || session_token.as_deref() != header_token {
            return (axum::http::StatusCode::FORBIDDEN, "CSRF token invalid").into_response();
        }
    } 

    next.run(req).await
}
