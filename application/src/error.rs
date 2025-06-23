use thiserror::Error;

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