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
    pub fn new(username: String, email: String, password: String) -> anyhow::Result<Self> {
        Ok(Self {
            id: Uuid::new_v4(),
            username,
            email,
            password: bcrypt::hash(password, bcrypt::DEFAULT_COST)?,
            is_active: false, // User starts as inactive until explicitly activated
            created_at: Some(chrono::Local::now().naive_local()),
            updated_at: Some(chrono::Local::now().naive_local()),
            deleted_at: None,
        })
    }

    // Method to deactivate the users
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    // Method to activate the users
    pub fn activate(&mut self) {
        self.is_active = true;
    }

    pub fn match_password(&self, password: &str) -> bool {
        bcrypt::verify(password, &self.password).unwrap_or(false)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserInfo {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub profile_picture: Option<String>,
}

pub trait UserInfoDisplay {
    fn get_profile_picture(&self) -> String;
    fn get_full_name(&self) -> String;
    fn get_default_profile_picture(&self) -> String;
    fn get_user_name(&self) -> String;
}

impl UserInfoDisplay for UserInfo {
    fn get_profile_picture(&self) -> String {
        match &self.profile_picture {
            Some(picture) => picture.to_string(),
            None => self.get_default_profile_picture(),
        }
    }
    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
    fn get_default_profile_picture(&self) -> String {
        format!(
            "https://ui-avatars.com/api/?name={}&background=random&rounded=true",
            self.get_full_name()
        )
    }
    fn get_user_name(&self) -> String {
        self.username.to_owned()
    }
}

impl UserInfo {
    pub fn new(
        id: Uuid,
        username: String,
        first_name: String,
        last_name: String,
        profile_picture: Option<String>,
    ) -> Self {
        Self {
            id,
            username,
            first_name,
            last_name,
            profile_picture,
        }
    }
}
