use crate::entity::{Chat, ChatMember, ChatPreview};
use async_trait::async_trait;
use log::info;
use persistence::DatabaseInterface;
use shaku::{Component, Interface};
use sqlx::{Error, SqliteConnection};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

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
        info!(
            "Initiating private chat between {} and {}",
            user1_id, user2_id
        );
        let mut pool = self.db.get_pool().begin().await?;
        let chat = Self::create_chat(user1_id, user2_id, &mut *pool).await?;
        Self::create_chat_member(user1_id, &mut *pool, &chat).await?;
        Self::create_chat_member(user2_id, &mut *pool, &chat).await?;
        pool.commit().await?;
        Ok("".to_string())
    }

    async fn get_user_chat_list(&self, _user_id: &str) -> anyhow::Result<Vec<ChatPreview>> {
        todo!()
    }
}

impl ChatService {
    async fn create_chat_member(
        user1_id: &str,
        pool: &mut SqliteConnection,
        chat: &Chat,
    ) -> anyhow::Result<()> {
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

        let user1_id = Uuid::from_str(user1_id)?;
        let member1 = ChatMember::new(chat.id, user1_id);
        sqlx::query(query)
            .bind(member1.id.to_string())
            .bind(member1.chat_id.to_string())
            .bind(member1.user_id.to_string())
            .bind(member1.joined_at)
            .execute(pool)
            .await?;

        Ok(())
    }

    async fn create_chat(
        user1_id: &str,
        user2_id: &str,
        pool: &mut SqliteConnection,
    ) -> Result<Chat, Error> {
        let chat = Chat::from_user1and2(user1_id, user2_id);
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
            .bind(chat.name.clone())
            .bind(chat.is_group)
            .bind(chat.created_at)
            .bind(chat.updated_at)
            .execute(pool)
            .await?;
        Ok(chat)
    }
}
