use sqlx::sqlite::SqliteRow;
use uuid::Uuid;
use sqlx::Row;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserDetail {
    pub id: Uuid,
    pub user_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<chrono::NaiveDate>,
    pub gender: Option<String>,
    pub profile_picture: Option<String>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

impl UserDetail {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            first_name: String::new(),
            last_name: String::new(),
            date_of_birth: None,
            gender: None,
            profile_picture: None,
            created_at: Some(chrono::Local::now().naive_local()),
            updated_at: Some(chrono::Local::now().naive_local()),
        }
    }

    pub fn from(row: SqliteRow) -> anyhow::Result<UserDetail> {
        Ok(UserDetail {
            id: row.try_get::<String, _>("id")?.parse()?,
            user_id: row.try_get::<String, _>("user_id")?.parse()?,
            first_name: row.try_get("first_name")?,
            last_name: row.try_get("last_name")?,
            date_of_birth: row.try_get("date_of_birth")?,
            gender: row.try_get("gender")?,
            profile_picture: row.try_get("profile_picture")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}
