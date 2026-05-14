#![forbid(unsafe_code)]

pub(crate) mod config;
pub(crate) mod errors;
pub(crate) mod store;

pub use config::DatabaseConfig;
pub use errors::Error;
pub use sqlx::PgPool;
pub use store::Store;

use sqlx::postgres::PgPoolOptions;

pub async fn init(config: &DatabaseConfig) -> Result<PgPool, Error> {
    let pool = PgPoolOptions::new()
        .connect_with(config.connect_options())
        .await?;

    Ok(pool)
}
