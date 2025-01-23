#[cfg(test)]
mod tests {
    use credentials::credential_services::{CredentialService, CredentialServiceInterface};
    use crypto::{Crypto, Encrypt};
    use mail::{Mail, SendEmail};
    use persistence::env::myenv::EnvInterface;
    use persistence::{DatabaseInterface, Env};
    use std::sync::Arc;
    use usecases::utils::setup_db;
    use usecases::{RegisterRequest, RegisterUseCase, RegisterUseCaseInterface};
    use users::user::User;
    use users::user_services::{UserService, UserServiceInterface};

    #[tokio::test]
    async fn test_register_usecase() {
        dotenv::dotenv().ok();
        // let env: &'static Env = Box::leak(Box::new(Env::new()));
        let env: Arc<dyn EnvInterface> = Arc::new(Env::new());
        let db = setup_db().await;
        let mail: Arc<dyn SendEmail> = Mail::new_arc(Arc::clone(&env));
        let user_service: Arc<dyn UserServiceInterface> =
            Arc::new(UserService::new(Arc::clone(&db)));
        let credential_service: Arc<dyn CredentialServiceInterface> =
            Arc::new(CredentialService::new(Arc::clone(&db)));
        let encrypt: Arc<dyn Encrypt> = Crypto::new_arc(env.clone());

        let register_usecase = RegisterUseCase::new(
            Arc::clone(&user_service),
            Arc::clone(&credential_service),
            Arc::clone(&mail),
            Arc::clone(&env),
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
        let env: Arc<dyn EnvInterface> = Arc::new(Env::new()); // let env: &'static Env = Arc::new(Env::new());
        let db: Arc<dyn DatabaseInterface> = setup_db().await;
        let mail: Arc<dyn SendEmail> = Mail::new_arc(env.clone());
        let user_service: Arc<dyn UserServiceInterface> = Arc::new(UserService::new(db.clone()));
        let credential_service: Arc<dyn CredentialServiceInterface> =
            Arc::new(CredentialService::new(Arc::clone(&db)));
        let encrypt: Arc<dyn Encrypt> = Crypto::new_arc(env.clone());
        let register_usecase = RegisterUseCase::new(
            Arc::clone(&user_service),
            Arc::clone(&credential_service),
            Arc::clone(&mail),
            Arc::clone(&env),
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
