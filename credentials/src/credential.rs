use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Credential {
    pub id: Uuid,
    pub user_id: Uuid,
    pub private_key: String,
    pub public_key: String,
    pub r#type: String, // Use `r#type` because `type` is a reserved keyword
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl Credential {
    pub fn new(
        user_id: Uuid,
        private_key: String,
        public_key: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            private_key,
            public_key,
            r#type: String::from("CHAT_KEY"),
            created_at: Some(chrono::Local::now().naive_local()),
            updated_at: Some(chrono::Local::now().naive_local()),
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_credential_new() {
        let user_id = Uuid::new_v4();
        let private_key = "private_key_example".to_string();
        let public_key = "public_key_example".to_string();

        let credential = Credential::new(user_id, private_key.clone(), public_key.clone());

        assert_eq!(credential.user_id, user_id);
        assert_eq!(credential.private_key, private_key);
        assert_eq!(credential.public_key, public_key);
        assert_eq!(credential.r#type, "CHAT_KEY");
        assert!(credential.created_at.is_some());
        assert!(credential.updated_at.is_some());
    }
}
