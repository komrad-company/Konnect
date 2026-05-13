use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database connection failed: {0}")]
    ConnectionError(#[from] sqlx::Error),
    #[error("Invalid database configuration: {reason}")]
    InvalidConfiguration { reason: String },
}
