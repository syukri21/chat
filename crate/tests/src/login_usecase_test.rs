#[cfg(test)]
mod tests {
    use credentials::credential::Credential;
    use credentials::credential_services::{CredentialService, CredentialServiceInterface};
    use jwt::JWT;
    use persistence::db::database::DBParameters;
    use persistence::db::sqlite::create_sqlite_db_pool;
    use persistence::env::myenv::EnvInterface;
    use persistence::{DatabaseInterface, Env, DB};
    use shaku::{module, HasComponent};
    use std::sync::Arc;
    use usecases::{LoginRequest, LoginUseCase, LoginUseCaseInterface};
    use users::user::User;
    use users::user_services::{UserService, UserServiceInterface};

    module! {
        TestModule {
            components = [LoginUseCase, UserService, CredentialService, Env, DB, JWT],
            providers = []
        }
    }

    async fn setup() -> TestModule {
        pretty_env_logger::init();
        let env = Env::load();

        let pool = Arc::new(create_sqlite_db_pool(env.get_db_url()).await.unwrap());
        let module = TestModule::builder()
            .with_component_parameters::<DB>(DBParameters {
                pool: Some(pool.clone()),
            })
            .with_component_override::<dyn EnvInterface>(Box::new(env))
            .build();
        let db: &dyn DatabaseInterface = module.resolve_ref();
        db.migrate().await;
        module
    }

    #[tokio::test]
    async fn test_all_login_usecase() {
        let module = setup().await;
        let (result_login_usecase, result_login_with_invalid_password) = futures::future::join(
            test_login_usecase(&module),
            test_login_usecase_with_invalid_password(&module),
        )
        .await;
        // Process results
        match result_login_usecase {
            Ok(_) => println!("Task Login usecase completed"),
            Err(e) => assert!(false, "error: {}", e),
        }

        match result_login_with_invalid_password {
            Ok(_) => println!("Task Login usecase with invalid password completed"),
            Err(e) => assert!(false, "error: {}", e),
        }

        println!("All tasks completed.");
    }

    // #[tokio::test]
    async fn test_login_usecase(module: &TestModule) -> anyhow::Result<()> {
        println!("test_login_usecase");
        let user_service: &dyn UserServiceInterface = module.resolve_ref();
        let login_usecase: &dyn LoginUseCaseInterface = module.resolve_ref();
        let credential_service: &dyn CredentialServiceInterface = module.resolve_ref();

        let user = User::new(
            String::from("syukri1"),
            String::from("syukrihsb148test@gmail.com"),
            String::from("password8"),
        )?;
        user_service.create_user(&user).await?;
        credential_service
            .create_credential(&Credential::new(
                user.id,
                "private_key_example",
                "public_key_example",
            ))
            .await?;

        let request = LoginRequest {
            username: "syukri1",
            password: "password8",
        };
        let response = login_usecase.login(request).await.unwrap();
        assert!(response.token.len() > 0, "token should not be empty");
        assert!(
            response.private_key.len() > 0,
            "private key should not be empty"
        );
        assert!(
            response.public_key.len() > 0,
            "public key should not be empty"
        );
        assert_eq!(
            response.public_key, "public_key_example",
            "public key should be equal"
        );
        assert_eq!(
            response.private_key, "private_key_example",
            "private key should be equal",
        );
        Ok(())
    }

    async fn test_login_usecase_with_invalid_password(module: &TestModule) -> anyhow::Result<()> {
        println!("test_login_usecase_with_invalid_password");
        let user_service: &dyn UserServiceInterface = module.resolve_ref();
        let login_usecase: &dyn LoginUseCaseInterface = module.resolve_ref();
        let credential_service: &dyn CredentialServiceInterface = module.resolve_ref();

        let user = User::new(
            String::from("syukritest"),
            String::from("syukrihsb148@gmail.com"),
            String::from("password8"),
        )
        .unwrap();
        user_service.create_user(&user).await.unwrap();
        credential_service
            .create_credential(&Credential::new(
                user.id,
                "private_key_example",
                "public_key_example",
            ))
            .await
            .unwrap();

        let request = LoginRequest {
            username: "syukritest",
            password: "invalid_password",
        };
        let response = login_usecase.login(request).await;
        assert!(response.is_err(), "response should be error");
        assert!(
            response.unwrap_err().to_string().contains("Login failed"),
            "error message should contain 'Login failed'"
        );
        Ok(())
    }
}
