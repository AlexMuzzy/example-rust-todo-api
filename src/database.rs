use sqlx::{PgPool, Pool, Postgres};
use std::env;

pub type DbPool = Pool<Postgres>;

pub async fn create_connection_pool() -> Result<DbPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://todo_user:todo_password@localhost:5432/todo_db".to_string()
    });

    let pool = PgPool::connect(&database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(pool)
}
