use std::future::Future;

use sqlx::PgPool;

use crate::errors::Error;

pub trait Store: Sized {
    fn new(pool: PgPool) -> Self;
    fn pool(&self) -> &PgPool;
    fn migrate(&self) -> impl Future<Output = Result<(), Error>> + Send + '_;
}
