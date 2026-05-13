use sqlx::PgPool;

pub trait Store: Sized {
    fn new(pool: PgPool) -> Self;
    fn pool(&self) -> &PgPool;
}
