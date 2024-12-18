#[cfg(test)]
mod tests {
    use users::user::User;
    use users::user_services::UserService;
    use crate::utils::setup;

    #[tokio::test]
    async fn test_create_user() {
        let db = setup().await;
        let user_service = UserService::new(db);
        {
            let user = User::new(
                String::from("testnumber1"),
                String::from("testnumber1@test.com"),
                String::from("testpassword"),
            );
            let result = user_service.create_user(user).await;
            assert_eq!(result.unwrap(), 1);
        }

        {
            let user = User::new(
                String::from("testnumber2"),
                String::from("testnumber2@test.com"),
                String::from("testpassword"),
            );
            let result = user_service.create_user(user).await;
            assert_eq!(result.unwrap(), 2);
        }
    }

    #[tokio::test]
    async fn test_get_user_by_userid() {
        let db = setup().await;
        let user_service = UserService::new(db);

        // First, create a user to test retrieving it
        let created_user = User::new(
            String::from("test_get_user"),
            String::from("test_get_user@test.com"),
            String::from("testpassword"),
        );
        let result = user_service.create_user(created_user.clone()).await;
        assert!(result.is_ok());

        // Now, fetch the user by ID
        let fetched_user = user_service
            .get_user_by_uuid(created_user.id)
            .await
            .unwrap();

        // Assert that fetched user matches the created user's details
        assert_eq!(fetched_user.id, fetched_user.id);
        assert_eq!(fetched_user.username, created_user.username);
        assert_eq!(fetched_user.email, created_user.email);
        assert_eq!(fetched_user.password, created_user.password);
        assert_eq!(fetched_user.is_active, created_user.is_active);

        // Check that created_at and updated_at are not None
        assert!(fetched_user.created_at.is_some());
        assert!(fetched_user.updated_at.is_some());
        assert!(fetched_user.deleted_at.is_none());
    }

    #[tokio::test]
    async fn test_get_user_by_username() {
        let db = setup().await;
        let user_service = UserService::new(db);

        // First, create a user to test retrieving it
        let created_user = User::new(
            String::from("testuser"),
            String::from("testuser@test.com"),
            String::from("testpassword"),
        );
        let result = user_service.create_user(created_user.clone()).await;
        assert!(result.is_ok());

        // Now, fetch the user by username
        let fetched_user = user_service.get_user_by_username("testuser").await.unwrap();

        // Assert that fetched user matches the created user's details
        assert_eq!(fetched_user.username, created_user.username);
        assert_eq!(fetched_user.email, created_user.email);
        assert_eq!(fetched_user.password, created_user.password);
        assert_eq!(fetched_user.is_active, created_user.is_active);

        // Check that created_at and updated_at are not None
        assert!(fetched_user.created_at.is_some());
        assert!(fetched_user.updated_at.is_some());
        assert!(fetched_user.deleted_at.is_none());
    }

    #[tokio::test]
    async fn test_activate_eligible_user() {
        let db = setup().await;
        let user_service = UserService::new(db);

        // First, create a user to test activating them
        let created_user = User::new(
            String::from("testactivateuser"),
            String::from("testactivateuser@test.com"),
            String::from("testpassword"),
        );
        let result = user_service.create_user(created_user.clone()).await;
        assert!(result.is_ok());

        // Activate the user
        let activation_result = user_service.activate_user(created_user.id).await;
        assert!(activation_result.is_ok());

        // Fetch the user and check if they are activated
        let activated_user = user_service
            .get_user_by_uuid(created_user.id)
            .await
            .unwrap();
        assert!(activated_user.is_active);
    }

    #[tokio::test]
    async fn test_activate_user_id_not_found_should_fail() {
        let db = setup().await;
        let user_service = UserService::new(db);

        // Attempt to activate a user with a non-existent ID
        let nonexistent_id = uuid::Uuid::new_v4(); // Generate a random UUID for test
        let activation_result = user_service.activate_user(nonexistent_id).await;

        // Assert that the operation failed with the expected error
        assert!(activation_result.is_err());

        // Extract the error and check its content
        let error_message = activation_result.unwrap_err().to_string();
        assert!(error_message.contains("User with userid"));
        assert!(error_message.contains("not found"));
        assert!(error_message.contains(nonexistent_id.to_string().as_str()));
    }

}
