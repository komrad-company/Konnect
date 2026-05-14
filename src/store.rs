use sqlx::PgPool;

pub trait Store {
    fn new(pool: PgPool) -> Self;
    fn pool(&self) -> &PgPool;
}
