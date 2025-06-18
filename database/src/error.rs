#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Failed to connect to the database: {0}")]
    ConnectionError(#[from] sqlx::Error),
    #[error("Migration error: {0}")]
    MigrationError(String),
}
