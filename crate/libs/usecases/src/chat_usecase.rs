use std::sync::Arc;

use chats::{chat_services::ChatServiceInterface, entity::ChatMessages};
use shaku::{Component, Interface};

#[derive(Component)]
#[shaku(interface = ChatUsecase)]
pub struct ChatUsecaseImpl {
    #[shaku(inject)]
    chats_service: Arc<dyn ChatServiceInterface>,
}

#[async_trait::async_trait]
pub trait ChatUsecase: Interface {
    async fn get_messages_of_chat(&self, chat_id: &str) -> anyhow::Result<ChatMessages>;
}

#[async_trait::async_trait]
impl ChatUsecase for ChatUsecaseImpl {
    async fn get_messages_of_chat(&self, chat_id: &str) -> anyhow::Result<ChatMessages> {
        self.chats_service.get_messages_of_chat(chat_id).await
    }
}
