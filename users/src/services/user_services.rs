use crate::models::user::User;
use chrono::NaiveDateTime;
use persistence::DatabaseInterface;
use std::sync::Arc;
use sqlx::Row;
use uuid::Uuid;

pub struct UserService {
    db: Arc<dyn DatabaseInterface>,
}

impl UserService {
    pub fn new(db: Arc<dyn DatabaseInterface>) -> Self {
        Self { db }
    }

    pub async fn create_user(&self, user: User) -> anyhow::Result<i64> {
        let mut connection = self.db.get_pool().acquire().await?;

        let query = r#"INSERT INTO users (
            id, username, email, password, pub_key, is_active, created_at, updated_at, deleted_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#;

        let id = sqlx::query(query)
            .bind(user.id.to_string())
            .bind(user.username)
            .bind(user.email)
            .bind(user.password) // Missing binding for password field
            .bind(user.pub_key) // Missing binding for pub_key field
            .bind(user.is_active) // Missing binding for is_active field
            .bind(user.created_at.map(|dt| dt)) // Set created_at, using provided or default value
            .bind(user.updated_at.map(|dt| dt))
            .bind(None::<NaiveDateTime>)
            .execute(&mut *connection)
            .await?
            .last_insert_rowid();

        Ok(id)
    }

    pub async fn get_user(&self, id: Uuid) -> anyhow::Result<User> {
        let mut connection = self.db.get_pool().acquire().await?;

        let query = r#"SELECT
            id,
            username,
            email,
            password,
            is_active,
            pub_key,
            created_at,
            updated_at,
            deleted_at
        FROM users
        WHERE id = ?"#;

        let row = sqlx::query(query)
            .bind(id.to_string())
            .fetch_one(&mut *connection)
            .await?;

        let user = User {
            id: row.try_get::<String, _>("id")?.parse()?,
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            password: row.try_get("password")?,
            is_active: row.try_get("is_active")?,
            pub_key: row.try_get("pub_key")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at").ok(),
        };

        Ok(user)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use persistence::{Env, DB};

    async fn setup() -> Arc<dyn DatabaseInterface> {
        let db_path = ":memory:"; // Use an in-memory database for tests
        let env = Env {
            db_url: format!("sqlite:{}", db_path),
        };
        let db = Arc::new(DB::new(env).await.unwrap());

        // Apply migrations
        let pool = db.get_pool();
        sqlx::migrate!("../migrations")
            .run(&*pool)
            .await
            .expect("Failed to run database migrations");

        db
    }

    #[tokio::test]
    async fn test_create_user() {
        let db = setup().await;
        let user_service = UserService::new(db);
        {
            let user = User::new(
                String::from("testnumber1"),
                String::from("testnumber1@test.com"),
                String::from("testpassword"),
                String::from("testpubkey"),
            );
            let result = user_service.create_user(user).await;
            assert_eq!(result.unwrap(), 1);
        }

        {
            let user = User::new(
                String::from("testnumber2"),
                String::from("testnumber2@test.com"),
                String::from("testpassword"),
                String::from("testpubkey"),
            );
            let result = user_service.create_user(user).await;
            assert_eq!(result.unwrap(), 2);
        }
    }
    
    #[tokio::test]
    async fn test_get_user_by_userid() {
        let db = setup().await;
        let user_service = UserService::new(db);

        // First, create a user to test retrieving it
        let created_user = User::new(
            String::from("test_get_user"),
            String::from("test_get_user@test.com"),
            String::from("testpassword"),
            String::from("testpubkey"),
        );
        let result = user_service.create_user(created_user.clone()).await;
        assert!(result.is_ok());

        // Now, fetch the user by ID
        let fetched_user = user_service.get_user(created_user.id).await.unwrap();

        // Assert that fetched user matches the created user's details
        assert_eq!(fetched_user.id, fetched_user.id);
        assert_eq!(fetched_user.username, created_user.username);
        assert_eq!(fetched_user.email, created_user.email);
        assert_eq!(fetched_user.password, created_user.password);
        assert_eq!(fetched_user.pub_key, created_user.pub_key);
        assert_eq!(fetched_user.is_active, created_user.is_active);

        // Check that created_at and updated_at are not None
        assert!(fetched_user.created_at.is_some());
        assert!(fetched_user.updated_at.is_some());
        assert!(fetched_user.deleted_at.is_none());
        
    }
}
