use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub mod models;



#[derive(Clone)]
pub struct Database {
    pub pool: Pool<Postgres>
}
impl Database {
    pub async fn new(url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .connect(url)
            .await.expect("Can not connect to database");

        Self { pool }
    }

    /*
        pub async fn get_user(&self, id: i32) -> Result<User, sqlx::Error> {
            return sqlx::query_as!(User, "SELECT id, name, created_at FROM users WHERE id = $1", id).fetch_one(&self.pool).await;
        }
     */
}