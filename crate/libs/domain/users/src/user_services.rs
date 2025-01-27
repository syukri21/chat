use crate::user::User;
use chrono::NaiveDateTime;
use persistence::DatabaseInterface;
use shaku::{Component, Interface};
use sqlx::sqlite::SqliteRow;
use sqlx::{Acquire, Row};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Component)]
#[shaku(interface = UserServiceInterface)]
pub struct UserService {
    #[shaku(inject)]
    db: Arc<dyn DatabaseInterface>,
}

#[async_trait::async_trait]
pub trait UserServiceInterface: Interface + Send + Sync {
    async fn create_user(&self, user: &User) -> anyhow::Result<i64>;
    async fn get_user_by_uuid(&self, id: Uuid) -> anyhow::Result<User>;
    async fn get_user_by_username(&self, username: &str) -> anyhow::Result<User>;
    async fn activate_user(&self, id: Uuid) -> anyhow::Result<()>;
}

impl UserService {
    pub fn new(db: Arc<dyn DatabaseInterface>) -> Self {
        Self { db }
    }
    fn row_to_user(row: SqliteRow) -> anyhow::Result<User> {
        let user = User {
            id: row.try_get::<String, _>("id")?.parse()?,
            username: row.try_get("username")?,
            email: row.try_get("email")?,
            password: row.try_get("password")?,
            is_active: row.try_get("is_active")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at").ok(),
        };

        Ok(user)
    }
}

#[async_trait::async_trait]
impl UserServiceInterface for UserService {
    async fn create_user(&self, user: &User) -> anyhow::Result<i64> {
        let mut connection = self.db.get_pool().acquire().await?;

        let query = r#"INSERT INTO users (
            id, username, email, password,  is_active, created_at, updated_at, deleted_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#;

        let id = sqlx::query(query)
            .bind(user.id.to_string())
            .bind(&user.username)
            .bind(&user.email)
            .bind(&user.password) // Missing binding for password field
            .bind(user.is_active) // Missing binding for is_active field
            .bind(user.created_at) // Set created_at, using provided or default value
            .bind(user.updated_at)
            .bind(None::<NaiveDateTime>)
            .execute(&mut *connection)
            .await?
            .last_insert_rowid();

        Ok(id)
    }
    async fn get_user_by_uuid(&self, id: Uuid) -> anyhow::Result<User> {
        let mut connection = self.db.get_pool().acquire().await?;

        let query = r#"SELECT
            id,
            username,
            email,
            password,
            is_active,
            created_at,
            updated_at,
            deleted_at
        FROM users
        WHERE id = ?"#;

        let row = sqlx::query(query)
            .bind(id.to_string())
            .fetch_one(&mut *connection)
            .await?;

        Self::row_to_user(row)
    }
    async fn get_user_by_username(&self, username_or_email: &str) -> anyhow::Result<User> {
        let mut connection = self.db.get_pool().acquire().await?;
        let query = r#"SELECT
            id,
            username,
            email,
            password,
            is_active,
            created_at,
            updated_at,
            deleted_at
            FROM users
            WHERE is_active = true and (username = ? or email = ?)"#;
        let row = sqlx::query(query)
            .bind(username_or_email.to_string())
            .bind(username_or_email.to_string())
            .fetch_one(&mut *connection)
            .await?;

        Self::row_to_user(row)
    }
    async fn activate_user(&self, id: Uuid) -> anyhow::Result<()> {
        let mut connection = self.db.get_pool().acquire().await?;
        let mut tx = connection.begin().await?;

        let query = r#"
        SELECT count(1) as count FROM users WHERE id = ?"#;
        let result = sqlx::query(query)
            .bind(id.to_string())
            .fetch_one(&mut *tx)
            .await;

        if result.is_err() {
            return Err(anyhow::format_err!("Something went wrong"));
        }

        let row = result?;
        let count: i64 = row.try_get("count")?;

        if count == 0 {
            return Err(anyhow::format_err!(
                "User with userid {} not found!!!",
                id.to_string()
            ));
        }

        let update_query = r#"
            UPDATE users
            SET is_active = true,
                updated_at = ?
            WHERE id = ?"#;

        sqlx::query(update_query)
            .bind(chrono::Utc::now().naive_utc())
            .bind(id.to_string())
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}
