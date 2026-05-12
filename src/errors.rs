use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Database connection failed: {0}")]
    ConnectionError(#[from] sqlx::Error),
    #[error("Migration failed: {0}")]
    MigrationError(#[from] sqlx::migrate::MigrateError),
    #[error("Invalid database configuration: {reason}")]
    InvalidConfiguration { reason: String },
}
