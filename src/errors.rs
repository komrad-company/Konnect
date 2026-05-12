use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database connection failed: {0}")]
    Connection(#[from] sqlx::Error),
    #[error("Invalid configuration format: {0}")]
    InvalidFormat(#[from] serde_json::Error),
    #[error("Invalid database configuration: {reason}")]
    InvalidConfiguration { reason: String },
}
