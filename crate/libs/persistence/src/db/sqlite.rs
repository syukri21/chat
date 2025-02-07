use sqlx::SqlitePool;

pub async fn create_sqlite_db_pool(database_url: &str) -> anyhow::Result<SqlitePool> {
    println!("Database URL: {}", database_url);
    let pool = SqlitePool::connect(database_url).await?;
    Ok(pool)
}
