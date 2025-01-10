use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chat {
    pub id: Uuid,
    pub name: String,
    pub is_group: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Default for Chat {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: String::new(),
            is_group: false,
            created_at: Option::from(chrono::Local::now().naive_local()),
            updated_at: Option::from(chrono::Local::now().naive_local()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub sender_id: Uuid,
    pub content: String,
    pub message_type: String,
    pub message_key: String,
    pub sent_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatMember {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub user_id: Uuid,
    pub joined_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageReadReceipt {
    pub id: Uuid,
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub read_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MessageReaction {
    pub id: Uuid,
    pub message_id: Uuid,
    pub user_id: Uuid,
    pub reaction: String,
    pub reacted_at: Option<chrono::NaiveDateTime>,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attachment {
    pub id: Uuid,
    pub message_id: Uuid,
    pub file_url: String,
    pub file_type: String,
    pub file_size: i32,
    pub uploaded_at: Option<chrono::NaiveDateTime>,
}

pub struct ChatPreview {
    pub chat_id: Uuid,
    pub name: String,
    pub is_group: bool,
    pub unread_message_count: i32,
    pub last_message: Option<Message>,
}
