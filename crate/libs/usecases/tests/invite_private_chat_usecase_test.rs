#[cfg(test)]
mod tests {
    use chats::chat_services::ChatService;
    use credentials::credential_services::CredentialService;
    use jwt::JWT;
    use log::info;
    use persistence::{DatabaseInterface, Env, DB};
    use shaku::{module, HasComponent};
    use std::sync::Once;
    use usecases::invite_private_chat_usecase::InvitePrivateChatUsecaseInterface;
    use usecases::{utils, InvitePrivateChatRequest, InvitePrivateChatUsecase};
    use user_details::user_detail_service::UserDetailServiceImpl;
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
            components = [UserDetailServiceImpl, InvitePrivateChatUsecase, UserService, ChatService, CredentialService, Env, DB, JWT],
            providers = []
        }
    }
    #[tokio::test]
    async fn test_invite_private_chat_usecase_should_faile_due_to_user_detail_not_found() {
        setup();
        info!("starting test_invite_private_chat_usecase");
        let env = Env::load_test();
        let module = utils::setup_module::<TestModule>(TestModule::builder(), env).await;
        let db: &dyn DatabaseInterface = module.resolve_ref();
        db.migrate().await;

        let user_service: &dyn UserServiceInterface = module.resolve_ref();
        let invite_private_chat_usecase: &dyn InvitePrivateChatUsecaseInterface =
            module.resolve_ref();

        let mut user1 = User::new(
            String::from("user1"),
            String::from("user1@gmail.com"),
            String::from("password8"),
        )
        .unwrap();
        user1.is_active = true;
        user_service.create_user(&user1).await.unwrap();

        let mut user2 = User::new(
            String::from("user2"),
            String::from("user2@gmail.com"),
            String::from("password8"),
        )
        .unwrap();
        user2.is_active = true;
        user_service.create_user(&user2).await.unwrap();

        let result = invite_private_chat_usecase
            .invite_private_chat(&InvitePrivateChatRequest {
                user_id: user1.id,
                user_email_or_username: "user2".to_string(),
            })
            .await;

        assert!(!result.is_ok(), "result should be ok");
    }
}
