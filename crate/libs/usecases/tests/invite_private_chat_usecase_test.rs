#[cfg(test)]
mod tests {
    use chats::chat_services::{ChatService, ChatServiceInterface};
    use credentials::credential_services::CredentialService;
    use jwt::JWT;
    use log::{error, info};
    use persistence::{DatabaseInterface, Env, DB};
    use shaku::{module, HasComponent};
    use std::sync::Once;
    use usecases::invite_private_chat_usecase::InvitePrivateChatUsecaseInterface;
    use usecases::{utils, InvitePrivateChatRequest, InvitePrivateChatUsecase};
    use users::user::User;
    use users::user_services::{UserService, UserServiceInterface};

    static INIT: Once = Once::new();

    /// Setup function that is only run once, even if called multiple times.
    fn setup() {
        INIT.call_once(|| {
            pretty_env_logger::init();
        });
    }

    module! {
         TestModule {
            components = [InvitePrivateChatUsecase, UserService, ChatService, CredentialService, Env, DB, JWT],
            providers = []
        }
    }
    #[tokio::test]
    async fn test_invite_private_chat_usecase() {
        setup();
        info!("starting test_invite_private_chat_usecase");
        // pretty_env_logger::init();
        let module = utils::setup_module::<TestModule>(TestModule::builder()).await;

        let db: &dyn DatabaseInterface = module.resolve_ref();
        db.migrate().await;

        let user_service: &dyn UserServiceInterface = module.resolve_ref();
        let invite_private_chat_usecase: &dyn InvitePrivateChatUsecaseInterface =
            module.resolve_ref();
        let chat_service: &dyn ChatServiceInterface = module.resolve_ref();

        let user1 = User::new(
            String::from("user1"),
            String::from("user1@gmail.com"),
            String::from("password8"),
        )
        .unwrap();
        user_service.create_user(&user1).await.unwrap();

        let user2 = User::new(
            String::from("user2"),
            String::from("user2@gmail.com"),
            String::from("password8"),
        )
        .unwrap();
        user_service.create_user(&user2).await.unwrap();

        let result = invite_private_chat_usecase
            .invite_private_chat(&InvitePrivateChatRequest {
                user_id: user1.id,
                user_email_or_username: "user2".to_string(),
            })
            .await;

        assert!(result.is_ok(), "result should be ok");
        let string = result.unwrap().to_string();
        assert!(!string.is_empty(), "chat id should not be empty");

        let chat_members = chat_service
            .get_chat_members(string.as_str())
            .await
            .map_err(move |e| {
                error!("failed to get chat members: {}", e);
                panic!("failed to get chat members: {}", e)
            })
            .unwrap();

        info!("chat members: {:#?}", chat_members);
        let iter = chat_members.into_iter();

        assert_eq!(
            iter.map(|v| v.user_id).collect::<Vec<_>>().as_slice(),
            &[user1.id, user2.id],
            "chat members should be user1 and user2"
        );
    }
}
