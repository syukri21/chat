#[cfg(test)]
mod tests {
    use crate::utils::setup_db;
    use credentials::credential_services::CredentialService;
    use crypto::Crypto;
    use mail::Mail;
    use persistence::Env;
    use std::sync::Arc;
    use usecases::{RegisterRequest, RegisterUseCase, RegisterUseCaseInterface};
    use users::user_services::UserService;

    #[tokio::test]
    async fn test_register_usecase() {
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
            &env,
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
}
