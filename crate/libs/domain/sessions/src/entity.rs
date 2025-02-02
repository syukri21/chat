use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Session {
    pub id: Uuid,
    pub session_id: Uuid,
    pub user_id: Uuid,
    pub user_agent: String,
    pub ip_address: String,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Session {
    pub fn new(session_id: Uuid, user_id: Uuid, user_agent: String, ip_address: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            session_id,
            user_id,
            user_agent,
            ip_address,
            created_at: Some(chrono::Local::now().naive_local()),
            updated_at: Some(chrono::Local::now().naive_local()),
        }
    }
}