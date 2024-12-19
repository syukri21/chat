#[cfg(test)]
mod tests {
    use users::user::User;
    use users::user_services::UserService;
    use crate::utils::setup_db;

    #[tokio::test]
    async fn test_create_user() {
        let db = setup_db().await;
        let user_service = UserService::new(db);

        let user = User::new(
            String::from("testnumber1"),
            String::from("testnumber1@test.com"),
            String::from("testpassword"),
        )
        .unwrap();

        let result = user_service.create_user(&user).await;
        assert_eq!(result.unwrap(), 1);

        let fetched_user = user_service.get_user_by_uuid(user.id).await.unwrap();
        assert_eq!(fetched_user.id, user.id);
        assert_eq!(fetched_user.username, user.username);
        assert_eq!(fetched_user.email, user.email);
        assert_eq!(fetched_user.password, user.password);
        assert_eq!(fetched_user.is_active, false);
        assert!(fetched_user.created_at.is_some());
        assert!(fetched_user.updated_at.is_some());
        assert!(fetched_user.deleted_at.is_none());
    }

    #[tokio::test]
    async fn test_get_user_by_userid() {
        let db = setup_db().await;
        let user_service = UserService::new(db);

        let user = User::new(
            String::from("test_get_user"),
            String::from("test_get_user@test.com"),
            String::from("testpassword"),
        )
        .unwrap();

        let result = user_service.create_user(&user).await;
        assert!(result.is_ok());

        let fetched_user = user_service.get_user_by_uuid(user.id).await.unwrap();
        assert_eq!(fetched_user.id, user.id);
        assert_eq!(fetched_user.username, user.username);
        assert_eq!(fetched_user.email, user.email);
        assert_eq!(fetched_user.password, user.password);
        assert_eq!(fetched_user.is_active, false);
        assert!(fetched_user.created_at.is_some());
        assert!(fetched_user.updated_at.is_some());
        assert!(fetched_user.deleted_at.is_none());
    }

    #[tokio::test]
    async fn test_get_user_by_username() {
        let db = setup_db().await;
        let user_service = UserService::new(db);

        let user = User::new(
            String::from("testuser"),
            String::from("testuser@test.com"),
            String::from("testpassword"),
        )
        .unwrap();

        let result = user_service.create_user(&user).await;
        assert!(result.is_ok());

        let fetched_user = user_service.get_user_by_username("testuser").await.unwrap();
        assert_eq!(fetched_user.username, user.username);
        assert_eq!(fetched_user.email, user.email);
        assert_eq!(fetched_user.password, user.password);
        assert_eq!(fetched_user.is_active, false);
        assert!(fetched_user.created_at.is_some());
        assert!(fetched_user.updated_at.is_some());
        assert!(fetched_user.deleted_at.is_none());
    }

    #[tokio::test]
    async fn test_activate_eligible_user() {
        let db = setup_db().await;
        let user_service = UserService::new(db);

        let user = User::new(
            String::from("testactivateuser"),
            String::from("testactivateuser@test.com"),
            String::from("testpassword"),
        )
        .unwrap();

        let result = user_service.create_user(&user).await;
        assert!(result.is_ok());

        let activation_result = user_service.activate_user(user.id).await;
        assert!(activation_result.is_ok());

        let activated_user = user_service.get_user_by_uuid(user.id).await.unwrap();
        assert!(activated_user.is_active);
    }

    #[tokio::test]
    async fn test_activate_user_id_not_found_should_fail() {
        let db = setup_db().await;
        let user_service = UserService::new(db);

        let nonexistent_id = uuid::Uuid::new_v4();

        let activation_result = user_service.activate_user(nonexistent_id).await;

        assert!(activation_result.is_err());

        let error_message = activation_result.unwrap_err().to_string();
        assert!(error_message.contains("User with userid"));
        assert!(error_message.contains("not found"));
        assert!(error_message.contains(nonexistent_id.to_string().as_str()));
    }

}
