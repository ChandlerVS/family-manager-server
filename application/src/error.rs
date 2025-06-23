use thiserror::Error;

#[cfg(feature = "axum")]
use axum::{
    response::IntoResponse,
    http::StatusCode,
    Json,
};

#[cfg(feature = "axum")]
use serde_json::json;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] database::error::DatabaseError),
    
    #[error("User already exists with email: {0}")]
    UserAlreadyExists(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Internal server error: {0}")]
    InternalError(String),
} 

#[cfg(feature = "axum")]
impl IntoResponse for ApplicationError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ApplicationError::DatabaseError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Internal server error"
                }))
            ).into_response(),
            ApplicationError::UserAlreadyExists(_) => (
                StatusCode::CONFLICT,
                Json(json!({
                    "error": "User already exists"
                }))
            ).into_response(),
            ApplicationError::InvalidInput(_) => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "Invalid input"
                }))
            ).into_response(),
            ApplicationError::InternalError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "error": "Internal server error"
                }))
            ).into_response(),
        }
    }
}
