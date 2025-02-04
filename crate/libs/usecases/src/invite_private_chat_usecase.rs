use chats::chat_services::ChatServiceInterface;
use commons::generic_errors::GenericError;
use log::{error, info};
use shaku::{Component, Interface};
use std::sync::Arc;
use users::{user::UserInfo, user_services::UserServiceInterface};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InvitePrivateChatRequest {
    pub user_id: Uuid,
    pub user_email_or_username: String,
}
#[async_trait::async_trait]
pub trait InvitePrivateChatUsecaseInterface: Interface + Send + Sync {
    async fn invite_private_chat(&self, request: &InvitePrivateChatRequest)
        -> anyhow::Result<Uuid>;
    async fn find_user_info_list(&self, query: &str) -> anyhow::Result<Vec<UserInfo>>;
}

#[derive(Component)]
#[shaku(interface = InvitePrivateChatUsecaseInterface)]
pub struct InvitePrivateChatUsecase {
    #[shaku(inject)]
    chats_service: Arc<dyn ChatServiceInterface>,
    #[shaku(inject)]
    user_service: Arc<dyn UserServiceInterface>,
}

#[async_trait::async_trait]
impl InvitePrivateChatUsecaseInterface for InvitePrivateChatUsecase {
    async fn invite_private_chat(
        &self,
        request: &InvitePrivateChatRequest,
    ) -> anyhow::Result<Uuid> {
        let target_user = self
            .user_service
            .get_user_by_username(request.user_email_or_username.as_str())
            .await
            .map_err(|e| GenericError::user_not_found(e))?;

        info!("target user found with id: {}", target_user.id);
        self.chats_service
            .initiate_private_chat(
                request.user_id.to_string().as_str(),
                target_user.id.to_string().as_str(),
            )
            .await
            .map_err(|e| GenericError::unknown(e))
    }
    async fn find_user_info_list(&self, query: &str) -> anyhow::Result<Vec<UserInfo>> {
        self.user_service
            .find_user_info_list(query)
            .await
            .map_err(|e| {
                error!("Error when getting user info list: {}", e);
                GenericError::unknown(e)
            })
    }
}
