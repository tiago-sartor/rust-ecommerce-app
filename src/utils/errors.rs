use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Session error: {0}")]
    Session(#[from] tower_sessions::session::Error),

    #[error("I/O error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Internal Server Error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Database(e) => {
                tracing::error!("Database error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "A database error occurred.").into_response()
            }
            AppError::Session(e) => {
                tracing::error!("Session error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "A session error occurred.").into_response()
            }
            AppError::IO(e) => {
                tracing::error!("I/O error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal I/O error occurred.").into_response()
            }
            AppError::Internal(e) => {
                tracing::error!("Internal server error: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "An internal server error occurred.").into_response()
            }
        }
    }
}
