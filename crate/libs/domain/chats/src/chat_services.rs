use crate::entity::{
    Chat, ChatMember, ChatMessages, ChatPreview, Message, MessageBox, MessageReaction,
    MessageReadReceipt,
};
use async_trait::async_trait;
use log::info;
use persistence::DatabaseInterface;
use shaku::{Component, Interface};
use sqlx::sqlite::SqliteRow;
use sqlx::{Error, Pool, Row, Sqlite, SqliteConnection};
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
pub trait ChatServiceInterface: Interface {
    async fn initiate_private_chat(&self, user1_id: &str, user2_id: &str) -> anyhow::Result<Uuid>;
    async fn get_user_chat_list(&self, user_id: &str) -> anyhow::Result<Vec<ChatPreview>>;
    async fn get_chat_members(&self, chat_id: &str) -> anyhow::Result<Vec<ChatMember>>;
    async fn is_chat_exist(&self, user1_id: &str, user2_id: &str) -> anyhow::Result<Option<Chat>>;
    async fn get_messages_of_chat(&self, chat_id: &str) -> anyhow::Result<ChatMessages>;
    async fn send_message_to_chat(
        &self,
        chat_id: &str,
        sender_id: &str,
        message: &str,
    ) -> anyhow::Result<MessageBox>;
}

#[derive(Component)]
#[shaku(interface = ChatServiceInterface)]
pub struct ChatService {
    #[shaku(inject)]
    db: Arc<dyn DatabaseInterface>,
}

#[async_trait]
impl ChatServiceInterface for ChatService {
    async fn initiate_private_chat(&self, user1_id: &str, user2_id: &str) -> anyhow::Result<Uuid> {
        info!(
            "Initiating private chat between {} and {}",
            user1_id, user2_id
        );
        let mut pool = self.db.get_pool().begin().await?;
        let chat = Self::create_chat(user1_id, user2_id, &mut pool).await?;
        info!("Chat created: {}", chat.id);
        Self::create_chat_member(user1_id, &mut pool, &chat).await?;
        info!("Chat member created for user1: {}", user1_id);
        Self::create_chat_member(user2_id, &mut pool, &chat).await?;
        info!("Chat member created for user2: {}", user2_id);
        pool.commit().await?;
        Ok(chat.id)
    }

    async fn get_user_chat_list(&self, _user_id: &str) -> anyhow::Result<Vec<ChatPreview>> {
        todo!()
    }

    async fn get_chat_members(&self, chat_id: &str) -> anyhow::Result<Vec<ChatMember>> {
        let mut pool = self.db.get_pool().acquire().await?;
        let query = r#"SELECT
            id,
            chat_id,
            user_id,
            joined_at
        FROM chat_members
        WHERE chat_id = ?"#;

        let rows = sqlx::query(query)
            .bind(chat_id.to_string())
            .fetch_all(&mut *pool)
            .await?;

        let mut members: Vec<ChatMember> = Vec::new();
        for row in rows {
            members.push(ChatMember {
                id: row.try_get::<String, _>("id")?.parse()?,
                chat_id: row.try_get::<String, _>("chat_id")?.parse()?,
                user_id: row.try_get::<String, _>("user_id")?.parse()?,
                joined_at: row.get("joined_at"),
            });
        }
        Ok(members)
    }

    async fn is_chat_exist(&self, user1_id: &str, user2_id: &str) -> anyhow::Result<Option<Chat>> {
        let mut pool = self.db.get_pool().acquire().await?;
        let chat = Chat::from_user1and2(user1_id, user2_id);
        let query = r#"
        SELECT
            id,
            name,
            is_group
        FROM chats
        WHERE
            name in (?, ?) 
        "#;

        let all_names = chat.get_all_possible_names();
        let rows = sqlx::query(query)
            .bind(all_names.get(0).unwrap())
            .bind(all_names.get(1).unwrap())
            .fetch_optional(&mut *pool)
            .await?;

        let rows: SqliteRow = match rows {
            Some(row) => row,
            None => return Ok(None),
        };

        let chat = Chat {
            id: rows.try_get::<String, _>("id")?.parse()?,
            name: rows.try_get::<String, _>("name")?,
            is_group: rows.try_get::<bool, _>("is_group")?,
            created_at: None,
            updated_at: None,
        };
        Ok(Some(chat))
    }

    async fn get_messages_of_chat(&self, chat_id: &str) -> anyhow::Result<ChatMessages> {
        let mut pool = self.db.get_pool().acquire().await?;
        let query = r#"SELECT
            id,
            chat_id,
            sender_id,
            content,
            message_type,
            message_key,
            sent_at
        FROM messages
        WHERE chat_id = ?
        ORDER BY created_at ASC 
        LIMIT 100"#;

        let rows = sqlx::query(query)
            .bind(chat_id.to_string())
            .fetch_all(&mut *pool)
            .await?;

        let mut messages: Vec<MessageBox> = Vec::new();
        let mut msg_ids: Vec<String> = Vec::new();

        for row in rows {
            let id: Uuid = row.try_get::<String, _>("id")?.parse()?;
            msg_ids.push(id.to_string());
            let message = Message {
                id,
                chat_id: row.try_get::<String, _>("chat_id")?.parse()?,
                sender_id: row.try_get::<String, _>("sender_id")?.parse()?,
                content: row.try_get::<String, _>("content")?.parse()?,
                message_type: row.try_get::<String, _>("message_type")?.parse()?,
                message_key: row.try_get::<String, _>("message_key")?.parse()?,
                sent_at: row.get("sent_at"),
            };

            let recipients = Vec::new();
            let receipts = Vec::new();
            let message_box = MessageBox(message, recipients, receipts);
            messages.push(message_box);
        }

        let msg_ids = Arc::new(msg_ids);
        let conn_pool = self.db.get_pool();
        let receipt_handler = tokio::spawn({
            let msg_ids = Arc::clone(&msg_ids);
            let conn_pool = conn_pool.clone();
            async move { Self::get_recipients_of_message(conn_pool, msg_ids).await }
        });

        let reaction_handler = tokio::spawn({
            let msg_ids = Arc::clone(&msg_ids);
            let conn_pool = conn_pool.clone();
            async move { Self::get_reactions_of_message(conn_pool, msg_ids).await }
        });

        //let read_receipts = Self::get_recipients_of_message(&mut pool, &msg_ids).await?;
        //let reactions = Self::get_reactions_of_message(&mut pool, &msg_ids).await?;

        let (read_receipts, reactions) = tokio::join!(receipt_handler, reaction_handler);
        let read_receipts = read_receipts??;
        let reactions = reactions??;

        for message in &mut messages {
            let message_id = message.0.id;
            if reactions.contains_key(&message_id) {
                message.2.push(reactions.get(&message_id).unwrap().clone());
            }
            if read_receipts.contains_key(&message_id) {
                message
                    .1
                    .push(read_receipts.get(&message_id).unwrap().clone());
            }
        }

        Ok(ChatMessages {
            chat_id: Uuid::from_str(chat_id)?,
            chat_name: String::new(),
            is_group: false,
            messages,
            chat_members: Vec::new(),
        })
    }

    async fn send_message_to_chat(
        &self,
        chat_id: &str,
        sender_id: &str,
        message: &str,
    ) -> anyhow::Result<MessageBox> {
        let mut pool = self.db.get_pool().acquire().await?;
        let chat_id = Uuid::from_str(chat_id)?;
        let sender_id = Uuid::from_str(sender_id)?;
        log::info!("Sending message to chat: {}, from sender: {}", chat_id, sender_id);
        let message = Message::new_private_message(chat_id, sender_id, message.to_owned());
        let query = r#"INSERT INTO messages (
            id,
            chat_id,
            sender_id,
            content,
            message_type,
            message_key,
            sent_at
        ) VALUES (
            ?,
            ?,
            ?,
            ?,
            ?,
            ?,
            ?
        )"#;

        sqlx::query(query)
            .bind(message.id.to_string())
            .bind(message.chat_id.to_string())
            .bind(message.sender_id.to_string())
            .bind(message.content.clone())
            .bind(message.message_type.clone())
            .bind(message.message_key.clone())
            .bind(message.sent_at)
            .execute(&mut *pool)
            .await?;

        // TODO: implement reaction and read receipt
        let recipients = Vec::new();
        let reactions = Vec::new();
        Ok(MessageBox(message, recipients, reactions))
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

    async fn get_recipients_of_message(
        pool: Arc<Pool<Sqlite>>,
        msg_ids: Arc<Vec<String>>,
    ) -> anyhow::Result<HashMap<Uuid, (String, MessageReadReceipt)>> {
        let mut pool = pool.acquire().await?;
        let placeholders = msg_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query = format!(
            r#"SELECT id, chat_id, sender_id, content, message_type, message_key, sent_at FROM messages WHERE id in ({}) "#,
            placeholders
        );

        let mut qx = sqlx::query(&query);
        for msg_id in msg_ids.iter() {
            qx = qx.bind(msg_id);
        }

        let rows = qx.fetch_all(&mut *pool).await?;
        let mut recipients = HashMap::new();
        for row in rows {
            let message_id = row.try_get::<String, _>("message_id")?.parse()?;
            let receipt = (
                ChatService::decide_name(&row)?,
                MessageReadReceipt {
                    id: row.try_get::<String, _>("id")?.parse()?,
                    message_id,
                    user_id: row.try_get::<String, _>("user_id")?.parse()?,
                    read_at: row.get("read_at"),
                },
            );
            recipients.insert(message_id, receipt);
        }

        Ok(recipients)
    }
    async fn get_reactions_of_message(
        pool: Arc<Pool<Sqlite>>,
        msg_ids: Arc<Vec<String>>,
    ) -> anyhow::Result<HashMap<Uuid, (String, MessageReaction)>> {
        let mut pool = pool.acquire().await?;
        let placeholders = msg_ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        let query = format!(
            r#"SELECT 
                    id,
                    message_id,
                    user_id,
                    reaction,
                    reacted_at,
                    user_details.first_name as first_name,
                    user_details.last_name as last_name,
                    users.username as username
             FROM message_read_receipts 
             LEFT JOIN user_details ON message_read_receipts.user_id = user_details.id
             LEFT JOUN users ON message_read_receipts.user_id = users.id
             WHERE id in ({}) "#,
            placeholders
        );

        let mut qx = sqlx::query(&query);
        for msg_id in msg_ids.iter() {
            qx = qx.bind(msg_id);
        }
        let rows = qx.fetch_all(&mut *pool).await?;
        let mut recipients = HashMap::new();

        for row in rows {
            let message_id = row.try_get::<String, _>("message_id")?.parse()?;
            let name = ChatService::decide_name(&row)?;
            recipients.insert(
                message_id,
                (
                    name,
                    MessageReaction {
                        id: row.try_get::<String, _>("id")?.parse()?,
                        message_id,
                        user_id: row.try_get::<String, _>("user_id")?.parse()?,
                        reaction: row.try_get::<String, _>("reaction")?.parse()?,
                        reacted_at: row.get("reacted_at"),
                    },
                ),
            );
        }

        Ok(recipients)
    }

    fn decide_name(row: &SqliteRow) -> Result<String, anyhow::Error> {
        let username = row.try_get::<String, _>("username")?.parse()?;
        let first_name: Result<String, _> = row.try_get::<String, _>("first_name")?.parse();
        let last_name: Result<String, _> = row.try_get::<String, _>("last_name")?.parse();
        let mut name = username;
        if first_name.is_ok() && last_name.is_ok() {
            name = format!("{} {}", first_name.unwrap(), last_name.unwrap());
        }
        Ok(name)
    }
}
