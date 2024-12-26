#[cfg(test)]
mod tests {
    use crate::utils::setup_db;
    use credentials::credential::Credential;
    use credentials::credential_services::CredentialService;
    use std::sync::Arc;
    use users::user::User;
    use users::user_services::{UserService, UserServiceInterface};

    #[tokio::test]
    async fn test_create_credential_and_get_credential_by_user_id() {
        let db = setup_db().await;
        let user_service = UserService::new(Arc::clone(&db));
        let user = User::new(
            String::from("testnumber1"),
            String::from("testnumber1@test.com"),
            String::from("testpassword"),
        )
        .unwrap();
        let user_id = user.id.clone();
        let result = user_service.create_user(&user).await;
        assert_eq!(result.unwrap(), 1);

        let credential_service = CredentialService::new(Arc::clone(&db));
        let credential = Credential::new(user_id, "private_key_example", "public_key_example");
        let credential_result = credential_service.create_credential(&credential).await;
        assert!(credential_result.is_ok());

        let fetched_credential = credential_service
            .get_credential_by_user_id(user_id)
            .await
            .unwrap();
        assert_eq!(fetched_credential.user_id, user_id);
        assert_eq!(fetched_credential.private_key, "private_key_example");
        assert_eq!(fetched_credential.public_key, "public_key_example");
        assert_eq!(fetched_credential.r#type, "CHAT_KEY");
    }
}
