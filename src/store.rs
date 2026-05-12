use sqlx::PgPool;

pub trait Store {
    fn pool(&self) -> &PgPool;
}
