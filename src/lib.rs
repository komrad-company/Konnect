#![forbid(unsafe_code)]

pub(crate) mod config;
pub(crate) mod errors;
pub(crate) mod store;

pub use chrono;
pub use config::DatabaseConfig;
pub use errors::Error;
pub use sqlx::{PgPool, FromRow};
pub use store::Store;
pub use uuid::Uuid;

use sqlx::postgres::PgPoolOptions;

pub async fn init(config: &DatabaseConfig) -> Result<PgPool, Error> {
    let pool = PgPoolOptions::new()
        .connect(&config.connection_url())
        .await?;

    Ok(pool)
}
