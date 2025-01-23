#[cfg(test)]
mod tests {
    use credentials::credential_services::CredentialService;
    use crypto::Crypto;
    use mail::Mail;
    use persistence::Env;
    use std::sync::Arc;
    use persistence::env::myenv::EnvInterface;
    use usecases::utils::setup_db;
    use usecases::{RegisterRequest, RegisterUseCase, RegisterUseCaseInterface};
    use users::user::User;
    use users::user_services::{UserService, UserServiceInterface};

    #[tokio::test]
    async fn test_register_usecase() {
        dotenv::dotenv().ok();
        let env: &'static Env = Box::leak(Box::new(Env::new()));
        let env1: dyn EnvInterface  = Env::new();
        let db = setup_db().await;
        let mail = Mail::new_arc(Arc::new(env));
        let user_service = Arc::new(UserService::new(Arc::clone(&db)));
        let credential_service = Arc::new(CredentialService::new(Arc::clone(&db)));
        let encrypt = Crypto::new_arc(&env);

        let register_usecase = RegisterUseCase::new(
            Arc::clone(&user_service),
            Arc::clone(&credential_service),
            Arc::clone(&mail),
            Arc::clone(env),
            Arc::clone(&encrypt),
        );

        let request = RegisterRequest {
            username: "syukri",
            email: "syukrihsb148@gmail.com",
            password: "password8",
            private_key: "private_key",
            public_key: "public_key",
        };

        let response = register_usecase.register(&request).await;
        assert!(response.is_ok());
    }

    #[tokio::test]
    async fn test_activate_user() {
        dotenv::dotenv().ok();
        let env: &'static Env = Box::leak(Box::new(Env::new()));
        let db = setup_db().await;
        let mail = Mail::new_arc(&env);
        let user_service = Arc::new(UserService::new(Arc::clone(&db)));
        let credential_service = Arc::new(CredentialService::new(Arc::clone(&db)));
        let encrypt = Crypto::new_arc(&env);
        let register_usecase = RegisterUseCase::new(
            Arc::clone(&user_service),
            Arc::clone(&credential_service),
            Arc::clone(&mail),
            Arc::clone(env),
            Arc::clone(&encrypt),
        );

        let user = User::new(
            String::from("syukri"),
            String::from("syukrihsb148@gmail.com"),
            String::from("password8"),
        )
        .unwrap();
        user_service.create_user(&user).await.unwrap();

        // test create user, ensure user is not active
        let x = user_service.get_user_by_uuid(user.id).await.unwrap();
        assert!(!x.is_active);

        // test activating user, make user status active
        let user_id = user.id.to_string();
        let user_id_encrypted = encrypt.encrypt(user_id.as_str()).await.unwrap();
        let response = register_usecase.activate_user(&user_id_encrypted).await;
        assert!(response.is_ok());

        // test activate user, ensure user is active
        let x = user_service.get_user_by_uuid(user.id).await.unwrap();
        assert!(x.is_active);
    }
}
