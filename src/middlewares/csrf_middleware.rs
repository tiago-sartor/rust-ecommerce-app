use crate::utils::errors::AppError;
use crate::utils::helpers;
use axum::{
    body::{Body, to_bytes},
    extract::Request,
    http::{Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Redirect, Response},
};
use std::collections::HashMap;
use tower_sessions::Session;

pub async fn csrf_middleware(session: Session, mut request: Request, next: Next) -> Result<Response, AppError> {
    let mut session_token: Option<String> = session.get("csrf_token").await?;

    // Initialize CSRF token if it doesn't exist in the session
    if session_token.is_none() {
        let token = helpers::generate_random_token(64);
        session.insert("csrf_token", token.clone()).await?;
        session_token = Some(token);
    }

    if matches!(request.method(), &Method::POST | &Method::PUT | &Method::DELETE | &Method::PATCH) {
        let mut request_token: Option<String> = None;

        let (parts, body) = request.into_parts();

        // Buffer the body bytes into memory.
        // In production, always set a sensible limit (e.g., 2MB) to avoid OOM attacks.
        match to_bytes(body, 2 * 1024 * 1024).await {
            Ok(bytes) => {
                if let Ok(params) = serde_urlencoded::from_bytes::<HashMap<String, String>>(&bytes) {
                    request_token = params.get("csrf_token").cloned();
                }
                // Reconstruct the request with the same bytes so the handler can read it again
                request = Request::from_parts(parts, Body::from(bytes));
            }
            Err(_) => return Ok((StatusCode::BAD_REQUEST, "Failed to read the request body.").into_response()),
        }

        if session_token.is_none() || request_token.is_none() || session_token != request_token {
            let path = request.uri().path_and_query().map(|pq| pq.as_str()).unwrap_or("/");
            return Ok(Redirect::to(path).into_response());
        }
    }

    Ok(next.run(request).await)
}
