use crate::models::user::User;
use persistence::DatabaseInterface;
use std::sync::Arc;
use chrono::NaiveDateTime;

pub struct UserService {
    db: Arc<dyn DatabaseInterface>,
}

impl UserService {
    pub fn new(db: Arc<dyn DatabaseInterface>) -> Self {
        Self { db }
    }

    ///
    ///
    /// # Arguments
    ///
    /// Creates a new user in the database.
    ///
    /// # Arguments
    ///
    /// * `user` - A `User` struct containing the necessary details for the user to be created.
    ///
    /// # Returns
    ///
    /// * `anyhow::Result<()>` - Returns `Ok(())` if the user is successfully created, or an error if the operation fails.
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// ```
    /// * `user`:
    ///
    /// returns: Result<(), Error>
    ///
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
            .await?.last_insert_rowid();

        Ok(id)
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
}
