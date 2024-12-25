use persistence::DB;
use shaku::{module, Component, Interface};
use std::sync::Arc;
use users::user_services::{UserService, UserServiceInterface};

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
        self.user_service.get_user_by_username("sd").await?;
        Ok(LoginResponse {
            token: "token",
            private_key: "private_key",
            public_key: "public_key",
        })
    }
}

module! {
    MyModule {
        components = [LoginUseCase, UserService, DB],
        providers = []
    }
}
