use crate::entity::{Chat, ChatPreview};
use async_trait::async_trait;
use persistence::DatabaseInterface;
use shaku::{Component, Interface};
use std::sync::Arc;

#[async_trait]
pub trait ChatServiceInterface: Interface {
    async fn initiate_private_chat(&self, user1_id: &str, user2_id: &str)
        -> anyhow::Result<String>;
    async fn get_user_chat_list(&self, user_id: &str) -> anyhow::Result<Vec<ChatPreview>>;
}

#[derive(Component)]
#[shaku(interface = ChatServiceInterface)]
pub struct ChatService {
    #[shaku(inject)]
    db: Arc<dyn DatabaseInterface>,
}

#[async_trait]
impl ChatServiceInterface for ChatService {
    async fn initiate_private_chat(
        &self,
        user1_id: &str,
        user2_id: &str,
    ) -> anyhow::Result<String> {
        let pool = self.db.get_pool().begin().await;
        let chat = Chat::default();
        let query = r#"INSERT INTO chats (
            id,
            name,
            is_group,
            created_at,
            updated_at
        ) VALUES (
            ?,
            ?,
            ?,
            ?,
            ?
        )"#;
        sqlx::query(query)
            .bind(chat.id.to_string())
            .bind(chat.name)
            .bind(chat.is_group)
            .bind(chat.created_at)
            .bind(chat.updated_at)
            .execute(&pool)
            .await?;

        let query = r#"INSERT INTO chat_members (
            id,
            chat_id,
            user_id,
            joined_at
        ) VALUES (
            ?,
            ?,
            ?,
            ?
        )"#;

        sqlx::query(query)
            .bind(uuid::Uuid::new_v4().to_string())
            .bind(chat.id.to_string())
            .bind(user2_id.to_string())
            .bind(chrono::Utc::now().naive_utc())
            .execute(&pool)
            .await?;

        Ok("".to_string())
    }

    async fn get_user_chat_list(&self, user_id: &str) -> anyhow::Result<Vec<ChatPreview>> {
        todo!()
    }
}
