use crate::credential::Credential;
use async_trait::async_trait;
use persistence::DatabaseInterface;
use shaku::{Component, Interface};
use sqlx::Row;
use std::sync::Arc;

#[derive(Component)]
#[shaku(interface = CredentialServiceInterface)]
pub struct CredentialService {
    #[shaku(inject)]
    db: Arc<dyn DatabaseInterface>,
}

#[async_trait::async_trait]
pub trait CredentialServiceInterface: Interface {
    async fn create_credential(&self, credential: &Credential) -> anyhow::Result<()>;
    async fn get_credential_by_user_id(&self, user_id: uuid::Uuid) -> anyhow::Result<Credential>;
}

impl CredentialService {
    pub fn new(db: Arc<dyn DatabaseInterface + Send + Sync>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl CredentialServiceInterface for CredentialService {
    async fn create_credential(&self, credential: &Credential) -> anyhow::Result<()> {
        let mut connection = self.db.get_pool().acquire().await?;
        let query = r#"
            INSERT INTO credentials (
                id, 
                user_id, 
                public_key,
                private_key,
                type, 
                created_at, 
                updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?)
        "#;

        sqlx::query(query)
            .bind(credential.id.to_string())
            .bind(credential.user_id.to_string())
            .bind(&credential.public_key)
            .bind(&credential.private_key)
            .bind(&credential.r#type)
            .bind(credential.created_at)
            .bind(credential.updated_at)
            .execute(&mut *connection)
            .await?;

        Ok(())
    }

    async fn get_credential_by_user_id(&self, user_id: uuid::Uuid) -> anyhow::Result<Credential> {
        let mut connection = self.db.get_pool().acquire().await?;
        let query = r#"SELECT
            id,
            user_id,
            public_key,
            private_key,
            type,
            created_at,
            updated_at
            FROM credentials
            WHERE user_id = ?"#;

        let row = sqlx::query(query)
            .bind(user_id.to_string())
            .fetch_one(&mut *connection)
            .await?;

        if row.is_empty() {
            return Err(anyhow::anyhow!(
                "Credential not found for user_id: {}",
                user_id
            ));
        }

        let credential = Credential {
            id: row.try_get::<String, _>("id")?.parse()?,
            user_id: row.try_get::<String, _>("user_id")?.parse()?,
            private_key: row.try_get("private_key")?,
            public_key: row.try_get("public_key")?,
            r#type: row.try_get("type")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        };

        Ok(credential)
    }
}
