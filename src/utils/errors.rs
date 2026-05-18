// use crate::emails::mailer::EmailError;
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
        // Log the error for debugging purposes
        tracing::error!(error = %self);

        match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "A database error occurred.").into_response(),
            AppError::Session(_) => (StatusCode::INTERNAL_SERVER_ERROR, "A session error occurred.").into_response(),
            AppError::IO(_) => (StatusCode::INTERNAL_SERVER_ERROR, "An internal I/O error occurred.").into_response(),
            AppError::Internal(_) => (StatusCode::INTERNAL_SERVER_ERROR, "An internal server error occurred.").into_response(),
        }
    }
}

// impl From<EmailError> for AppError {
//     fn from(error: EmailError) -> Self {
//         match error {
//             EmailError::TransportError(e) => AppError::Internal(e.to_string()),
//             EmailError::BuildError(e) => AppError::Internal(e.to_string()),
//             EmailError::AddressError(e) => AppError::Internal(e.to_string()),
//             EmailError::DatabaseError(e) => AppError::Database(e),
//         }
//     }
// }
