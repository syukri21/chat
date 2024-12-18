use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub is_active: bool,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

impl User {
    // Constructor to create a new User with a generated Uuid
    pub fn new(username: String, email: String, password: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            username,
            email,
            password,
            is_active: false, // User starts as inactive until explicitly activated
            created_at: Some(chrono::Local::now().naive_local()),
            updated_at: Some(chrono::Local::now().naive_local()),
            deleted_at: None,
        }
    }

    // Method to deactivate the users
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    // Method to activate the users
    pub fn activate(&mut self) {
        self.is_active = true;
    }
}
