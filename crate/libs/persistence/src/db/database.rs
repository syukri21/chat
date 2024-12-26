use crate::db::sqlite::create_sqlite_db_pool;
use crate::env::myenv::Env;
use shaku::{Component, Interface};
use sqlx::{Pool, Sqlite, SqlitePool};
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = DatabaseInterface)]
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

    pub async fn arc_new(env: Env) -> Arc<DB> {
        Arc::new(DB::new(env).await.unwrap())
    }

    pub async fn new_and_migrate(env: Env) -> Arc<DB> {
        let db = Self::arc_new(env).await;
        Self::migrate(&db).await;
        db
    }

    pub async fn migrate(&self) {
        let pool = self.get_pool();
        sqlx::migrate!("../../../migrations")
            .run(&*pool)
            .await
            .expect("Failed to run database migrations");
    }
}

// Update the `DatabaseInterface` to explicitly return `SqlitePool`
#[async_trait::async_trait]
pub trait DatabaseInterface: Interface + Send + Sync {
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
    use crate::env::myenv::Env;

    #[tokio::test]
    async fn test_db_new() {
        // Prepare a mock or testing database URL
        let test_db_url = "sqlite::memory:"; // In-memory SQLite for testing purposes

        let env = Env {
            db_url: test_db_url.to_string(),
            email_from: "".to_string(),
            email_from_email: "".to_string(),
            email_smtp_username: "".to_string(),
            email_smtp_password: "".to_string(),
            email_smtp_host: "".to_string(),
            email_smtp_port: "".to_string(),
            app_key_main: "".to_string(),
            app_callback_url: "".to_string(),
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
