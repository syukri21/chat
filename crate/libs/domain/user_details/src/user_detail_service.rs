use anyhow::anyhow;
use sqlx::Row;
use std::sync::Arc;

use persistence::DatabaseInterface;
use shaku::{Component, Interface};
use tracing::error;

use crate::entity::UserDetail;

#[derive(Component)]
#[shaku(interface = UserDetailService)]
pub struct UserDetailServiceImpl {
    #[shaku(inject)]
    db: Arc<dyn DatabaseInterface>,
}

#[async_trait::async_trait]
pub trait UserDetailService: Interface {
    async fn create_user_detail(&self, user_id: &UserDetail) -> anyhow::Result<()>;
    async fn update_user_detail(&self, user_detail: &UserDetail) -> anyhow::Result<()>;
    async fn is_user_detail_exist(&self, user_id: &str) -> anyhow::Result<bool>;
    async fn upsert_user_detail(&self, user_detail: &UserDetail) -> anyhow::Result<()>;
    async fn get_user_detail_by_user_id(&self, user_id: &str) -> anyhow::Result<UserDetail>;
}

#[async_trait::async_trait]
impl UserDetailService for UserDetailServiceImpl {
    async fn create_user_detail(&self, user_detail: &UserDetail) -> anyhow::Result<()> {
        let mut connection = self.db.get_pool().acquire().await?;

        let query = r#"INSERT INTO user_details (
        id,
        user_id,
        first_name,
        last_name,
        gender,
        date_of_birth,
        created_at, 
        updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#;
        sqlx::query(query)
            .bind(user_detail.id.to_string())
            .bind(user_detail.user_id.to_string())
            .bind(user_detail.first_name.clone())
            .bind(user_detail.last_name.clone())
            .bind(user_detail.gender.clone())
            .bind(user_detail.date_of_birth)
            .bind(user_detail.created_at)
            .bind(user_detail.updated_at)
            .execute(&mut *connection)
            .await
            .map_err(|e| {
                error!("Failed to create user_detail: {}", e);
                anyhow!("Failed to create user_detail")
            })?;

        Ok(())
    }

    async fn update_user_detail(&self, user_detail: &UserDetail) -> anyhow::Result<()> {
        let mut connection = self.db.get_pool().acquire().await?;
        let query = r#"UPDATE user_details SET 
        first_name = ?,
        last_name = ?,
        gender = ?,
        date_of_birth = ?,
        updated_at = ? WHERE user_id = ?"#;
        sqlx::query(query)
            .bind(user_detail.first_name.clone())
            .bind(user_detail.last_name.clone())
            .bind(user_detail.gender.clone())
            .bind(user_detail.date_of_birth)
            .bind(user_detail.updated_at)
            .bind(user_detail.user_id.to_string())
            .execute(&mut *connection)
            .await
            .map_err(|e| {
                error!("Failed to update user_detail: {}", e);
                anyhow!("Failed to update user_detail")
            })?;
        Ok(())
    }

    async fn is_user_detail_exist(&self, user_id: &str) -> anyhow::Result<bool> {
        let mut connection = self.db.get_pool().acquire().await?;
        let query = r#"SELECT count(1) as count FROM user_details WHERE user_id = ?"#;
        let results = sqlx::query(query)
            .bind(user_id.to_string())
            .fetch_one(&mut *connection)
            .await
            .map_err(|e| {
                error!("Failed to check user_detail: {}", e);
                anyhow!("Failed to check user_detail")
            })?;

        if results.is_empty() {
            return Ok(false);
        }

        let count: i64 = results.try_get::<i64, _>("count")?;
        Ok(count > 0)
    }

    async fn upsert_user_detail(&self, user_detail: &UserDetail) -> anyhow::Result<()> {
        let user_id = &user_detail.user_id.to_string();
        let exist = self.is_user_detail_exist(user_id.as_str()).await?;
        if exist {
            self.update_user_detail(user_detail).await
        } else {
            self.create_user_detail(user_detail).await
        }
    }

    async fn get_user_detail_by_user_id(&self, user_id: &str) -> anyhow::Result<UserDetail> {
        let mut connection = self.db.get_pool().acquire().await?;
        let query = r#"SELECT * FROM user_details WHERE user_id = ?"#;
        let results = sqlx::query(query)
            .bind(user_id.to_string())
            .fetch_one(&mut *connection)
            .await
            .map_err(|e| {
                error!("Failed to get user_detail: {}", e);
                anyhow!("Failed to get user_detail")
            })?;

        if results.is_empty() {
            return Err(anyhow!("User detail not found"));
        }

        UserDetail::from(results)
    }
}
