#[cfg(test)]
mod tests {
    use crate::utils::setup;
    use credentials::credential_services::CredentialService;
    use std::sync::Arc;
    use usecases::{RegisterRequest, RegisterResponse, RegisterUseCase, RegisterUseCaseInterface};
    use users::user_services::UserService;

    #[tokio::test]
    async fn test_register_usecase() {
        let db = setup().await;
        let user_service = Arc::new(UserService::new(Arc::clone(&db)));
        let credential_service = Arc::new(CredentialService::new(Arc::clone(&db)));
        let register_usecase =
            RegisterUseCase::new(Arc::clone(&user_service), Arc::clone(&credential_service));

        let request = RegisterRequest {
            username: "test",
            email: "test@example.com",
            password: "test",
            private_key: "private_key",
            public_key: "public_key",
        };

        let response = register_usecase.register(&request).await.unwrap();
        assert_eq!(response, RegisterResponse {});
    }
}
