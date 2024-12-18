use crate::db::sqlite::create_sqlite_db_pool;
use crate::env::env::Env;
use sqlx::{ Pool, Sqlite, SqlitePool};
use std::sync::Arc;

pub struct DB {
    pool: Arc<Pool<Sqlite>>,
}

impl DB {
    pub async fn new(env: Env) -> anyhow::Result<Self> {
        let pool = create_sqlite_db_pool(env.db_url.as_ref()).await?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}

// Update the `DatabaseInterface` to explicitly return `SqlitePool`
#[async_trait::async_trait]
pub trait DatabaseInterface {
    fn get_pool(&self) -> Arc<SqlitePool>;
}

impl DatabaseInterface for DB {
    fn get_pool(&self) -> Arc<SqlitePool> {
        Arc::clone(&self.pool)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::env::Env;

    #[tokio::test]
    async fn test_db_new() {
        // Prepare a mock or testing database URL
        let test_db_url = "sqlite::memory:"; // In-memory SQLite for testing purposes

        let env = Env {
            db_url: test_db_url.to_string(),
        };
        // Wrap it in an Arc and Box as required by the method signature

        // Call the `new` function and ensure it works properly
        let result = DB::new(env).await;

        // Assert that the result is Ok and contains a valid DB instance
        assert!(result.is_ok());
        let db = result.unwrap();
        assert!(!db.pool.is_closed());
    }
}
