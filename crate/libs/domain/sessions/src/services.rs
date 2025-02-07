use crate::entity::Session;
use log::error;
use persistence::DatabaseInterface;
use shaku::{Component, Interface};
use sqlx::Row;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Component)]
#[shaku(interface = SessionServiceInterface)]
pub struct SessionService {
    #[shaku(inject)]
    db: Arc<dyn DatabaseInterface>,
}

#[async_trait::async_trait]
pub trait SessionServiceInterface: Interface {
    async fn create_session(&self, session: &Session) -> anyhow::Result<()>;
    async fn check_session(&self, session_id: &str) -> anyhow::Result<bool>;
    async fn get_session(&self, session_id: &str) -> anyhow::Result<Option<Session>>;
    async fn delete_session(&self, session_id: &str) -> anyhow::Result<()>;
    async fn delete_sessions_by_user(&self, user_id: Uuid) -> anyhow::Result<()>;
}

#[async_trait::async_trait]
impl SessionServiceInterface for SessionService {
    async fn create_session(&self, session: &Session) -> anyhow::Result<()> {
        let mut connection = self.db.get_pool().acquire().await?;
        let query = r#"
            INSERT INTO sessions (id, session_id, user_id, user_agent, ip_address, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        "#;

        sqlx::query(query)
            .bind(session.id.to_string())
            .bind(session.session_id.to_string())
            .bind(session.user_id.to_string())
            .bind(&session.user_agent)
            .bind(&session.ip_address)
            .bind(session.created_at)
            .bind(session.updated_at)
            .execute(&mut *connection)
            .await
            .inspect_err(|e| {
                error!("Error occurred while creating session: {}", e.to_string());
            })?;

        Ok(())
    }

    async fn check_session(&self, session_id: &str) -> anyhow::Result<bool> {
        let mut connection = self.db.get_pool().acquire().await?;
        let query = r#"
            SELECT id
            FROM sessions
            WHERE session_id = ?
        "#;

        let session = sqlx::query(query)
            .bind(session_id.to_string())
            .fetch_one(&mut *connection)
            .await
            .inspect_err(|e| {
                error!("Error occurred while checking session: {}", e.to_string());
            })?;

        Ok(!session.is_empty())
    }

    async fn get_session(&self, session_id: &str) -> anyhow::Result<Option<Session>> {
        let mut connection = self.db.get_pool().acquire().await?;
        let query = r#"
            SELECT id, session_id, user_id, user_agent, ip_address, created_at, updated_at
            FROM sessions
            WHERE session_id = ?
        "#;

        let session = sqlx::query(query)
            .bind(session_id.to_string())
            .fetch_one(&mut *connection)
            .await
            .inspect_err(|e| {
                error!("Error occurred while getting session: {}", e.to_string());
            })?;

        if session.is_empty() {
            return Ok(None);
        }

        let session = Session {
            id: session.try_get::<String, _>("id")?.parse()?,
            session_id: session.try_get::<String, _>("session_id")?.parse()?,
            user_id: session.try_get::<String, _>("user_id")?.parse()?,
            user_agent: session.try_get("user_agent")?,
            ip_address: session.try_get("ip_address")?,
            created_at: session.try_get("created_at")?,
            updated_at: session.try_get("updated_at")?,
        };
        Ok(Some(session))
    }

    async fn delete_session(&self, session_id: &str) -> anyhow::Result<()> {
        let mut connection = self.db.get_pool().acquire().await?;
        let query = r#"
            DELETE FROM sessions
            WHERE session_id = ?
        "#;
        sqlx::query(query)
            .bind(session_id.to_string())
            .execute(&mut *connection)
            .await
            .inspect_err(|e| {
                error!("Error occurred while deleting session: {}", e.to_string());
            })?;
        Ok(())
    }

    async fn delete_sessions_by_user(&self, user_id: Uuid) -> anyhow::Result<()> {
        let mut connection = self.db.get_pool().acquire().await?;
        let query = r#"
            DELETE FROM sessions
            WHERE user_id = ?
        "#;
        sqlx::query(query)
            .bind(user_id.to_string())
            .execute(&mut *connection)
            .await
            .inspect_err(|e| {
                error!(
                    "Error occurred while deleting by users session: {}",
                    e.to_string()
                );
            })?;
        Ok(())
    }
}
