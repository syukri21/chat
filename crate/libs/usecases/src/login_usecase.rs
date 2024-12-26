use commons::generic_errors::GenericError;
use shaku::{Component, Interface};
use sqlx::Error;
use std::sync::Arc;
use users::user_services::UserServiceInterface;

#[derive(Component)]
#[shaku(interface = LoginUseCaseInterface)]
pub struct LoginUseCase {
    #[shaku(inject)]
    user_service: Arc<dyn UserServiceInterface>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginRequest<'a> {
    pub username: &'a str,
    pub password: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoginResponse<'a> {
    pub token: &'a str,
    pub private_key: &'a str,
    pub public_key: &'a str,
}

#[async_trait::async_trait]
pub trait LoginUseCaseInterface: Interface {
    async fn login(&self, request: LoginRequest<'_>) -> anyhow::Result<LoginResponse>;
}

#[async_trait::async_trait]
impl LoginUseCaseInterface for LoginUseCase {
    async fn login(&self, _request: LoginRequest<'_>) -> anyhow::Result<LoginResponse> {
        let result = self.user_service.get_user_by_username("sd").await;
        if result.is_err() {
            return match result.unwrap_err().downcast_ref::<Error>() {
                Some(Error::RowNotFound) => Err(GenericError::login_failed().into()),
                _ => Err(GenericError::unknown()),
            };
        }

        Ok(LoginResponse {
            token: "token",
            private_key: "private_key",
            public_key: "public_key",
        })
    }
}
#[cfg(test)]
mod tests {
    use crate::login_usecase::{LoginRequest, LoginUseCase, LoginUseCaseInterface};
    use commons::generic_errors::GenericError;
    use persistence::{DatabaseInterface, Env, DB};
    use shaku::{module, HasComponent};
    use users::user_services::UserService;

    module! {
        MyModule {
            components = [LoginUseCase, UserService, DB],
            providers = []
        }
    }

    #[tokio::test]
    async fn test_login_invalid_username() {
        let env = Env::load();
        let db = DB::new(env).await.unwrap();
        db.migrate().await;

        let module = MyModule::builder()
            .with_component_override::<dyn DatabaseInterface>(Box::new(db))
            .build();
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
