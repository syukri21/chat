use std::sync::Arc;

use async_trait::async_trait;
use commons::generic_errors::GenericError;
use shaku::{Component, Interface};
use user_details::{entity::UserDetail, user_detail_service::UserDetailService};

#[derive(Component)]
#[shaku(interface = UserDetailUsecase)]
pub struct UserDetailUsecaseImpl {
    #[shaku(inject)]
    user_detail_service: Arc<dyn UserDetailService>,
}

#[async_trait]
pub trait UserDetailUsecase: Interface {
    async fn update_profile(&self, user_detail: &UserDetail) -> anyhow::Result<()>;
}

#[async_trait]
impl UserDetailUsecase for UserDetailUsecaseImpl {
    async fn update_profile(&self, user_detail: &UserDetail) -> anyhow::Result<()> {
        self.user_detail_service
            .upsert_user_detail(user_detail)
            .await
            .map_err(|e| GenericError::unknown(e))
    }
}
