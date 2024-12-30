use commons::generic_errors::GenericError;
use credentials::credential_services::CredentialServiceInterface;
use jwt::{AccessClaims, JWTInterface, Role};
use shaku::{Component, Interface};
use sqlx::Error;
use std::sync::Arc;
use users::user_services::UserServiceInterface;

#[derive(Component)]
#[shaku(interface = LoginUseCaseInterface)]
pub struct LoginUseCase {
    #[shaku(inject)]
    user_service: Arc<dyn UserServiceInterface>,
    #[shaku(inject)]
    jwt_service: Arc<dyn JWTInterface>,
    #[shaku(inject)]
    credential_service: Arc<dyn CredentialServiceInterface>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

impl LoginRequest<'_> {
    async fn validate(&self) -> anyhow::Result<()> {
        if self.username.is_empty() {
            return Err(GenericError::invalid_input("Username is empty".to_string()));
        }

        if self.password.is_empty() {
            return Err(GenericError::invalid_input("Password is empty".to_string()));
        }

        let username_err = "Username must be at least 3 characters";
        if self.username.len() < 3 {
            return Err(GenericError::invalid_input(String::from(username_err)));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginResponse {
    pub token: String,
    pub private_key: String,
    pub public_key: String,
}

#[async_trait::async_trait]
pub trait LoginUseCaseInterface: Interface {
    async fn login(&self, request: LoginRequest<'_>) -> anyhow::Result<LoginResponse>;
}

#[async_trait::async_trait]
impl LoginUseCaseInterface for LoginUseCase {
    async fn login(&self, request: LoginRequest<'_>) -> anyhow::Result<LoginResponse> {
        request.validate().await?;

        let user = self
            .user_service
            .get_user_by_username(request.username)
            .await
            .map_err(|e| match e.downcast_ref::<Error>() {
                Some(Error::RowNotFound) => GenericError::login_failed(),
                _ => GenericError::unknown(e),
            })?;

        let credential = self
            .credential_service
            .get_credential_by_user_id(user.id)
            .await
            .map_err(|e| GenericError::unknown(e))?;

        let access_claim = AccessClaims::new(user.id.to_string(), Role::Admin);
        let token = self
            .jwt_service
            .generate_token(&access_claim)
            .await
            .map_err(|e| GenericError::unknown(e))?;

        Ok(LoginResponse {
            token: token.token.to_owned(),
            private_key: credential.private_key.to_owned(),
            public_key: credential.public_key.to_owned(),
        })
    }
}
#[cfg(test)]
mod tests {
    use crate::login_usecase::{LoginRequest, LoginUseCase, LoginUseCaseInterface};
    use commons::generic_errors::GenericError;
    use credentials::credential_services::CredentialService;
    use jwt::JWT;
    use persistence::db::database::DBParameters;
    use persistence::db::sqlite::create_sqlite_db_pool;
    use persistence::env::myenv::EnvInterface;
    use persistence::{DatabaseInterface, Env, DB};
    use shaku::{module, HasComponent};
    use std::sync::Arc;
    use users::user_services::UserService;

    module! {
        MyModule {
            components = [LoginUseCase, UserService, CredentialService, Env, DB, JWT],
            providers = []
        }
    }

    #[tokio::test]
    async fn test_login_invalid_username() {
        pretty_env_logger::init();
        let env = Env::load();

        let pool = Arc::new(create_sqlite_db_pool(env.get_db_url()).await.unwrap());
        let module = MyModule::builder()
            // .with_component_override::<dyn DatabaseInterface>(Box::new(db))
            .with_component_parameters::<DB>(DBParameters {
                pool: Some(pool.clone()),
            })
            .with_component_override::<dyn EnvInterface>(Box::new(env))
            .build();

        let db: &dyn DatabaseInterface = module.resolve_ref();
        db.migrate().await;

        let login_usecase: &dyn LoginUseCaseInterface = module.resolve_ref();
        let result = login_usecase
            .login(LoginRequest {
                username: "invalidusername",
                password: "passwordpassword1",
            })
            .await;
        assert!(result.is_err());
        match result.unwrap_err().downcast_ref::<GenericError>() {
            Some(GenericError::LoginFailed(u32)) => assert_eq!(*u32, 401),
            _ => panic!("error is not login failed"),
        };
    }
}
