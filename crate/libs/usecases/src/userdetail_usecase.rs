use std::sync::Arc;

use async_trait::async_trait;
use commons::generic_errors::GenericError;
use shaku::{Component, Interface};
use user_details::{entity::UserDetail, user_detail_service::UserDetailService};
use users::{user::User, user_services::UserServiceInterface};

#[derive(Component)]
#[shaku(interface = UserDetailUsecase)]
pub struct UserDetailUsecaseImpl {
    #[shaku(inject)]
    user_detail_service: Arc<dyn UserDetailService>,
    #[shaku(inject)]
    user_service: Arc<dyn UserServiceInterface>,
}

#[async_trait]
pub trait UserDetailUsecase: Interface {
    async fn update_profile(&self, user_detail: &UserDetail) -> anyhow::Result<()>;
    async fn get_user_info(&self, user_id: &str) -> anyhow::Result<UserInfo>;
}

#[async_trait]
impl UserDetailUsecase for UserDetailUsecaseImpl {
    async fn update_profile(&self, user_detail: &UserDetail) -> anyhow::Result<()> {
        self.user_detail_service
            .upsert_user_detail(user_detail)
            .await
            .map_err(|e| GenericError::unknown(e))
    }

    async fn get_user_info(&self, user_id: &str) -> anyhow::Result<UserInfo> {
        let user: User = self
            .user_service
            .get_user_by_uuid(user_id.parse()?)
            .await
            .map_err(|e| GenericError::unknown(e))?;

        let user_detail = self
            .user_detail_service
            .get_user_detail_by_user_id(user_id)
            .await
            .map_or(None, |e| Some(e));

        Ok(UserInfo {
            username: user.username,
            email: user.email,
            user_details: user_detail,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserInfo {
    pub username: String,
    pub email: String,
    pub user_details: Option<UserDetail>,
}
