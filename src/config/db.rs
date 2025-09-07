use sqlx::{mysql::MySqlPoolOptions, MySqlPool};
use std::env;

pub async fn get_db_pool() -> Result<MySqlPool, sqlx::Error> {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
