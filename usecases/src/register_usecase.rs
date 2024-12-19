use async_trait::async_trait;
use credentials::credential::Credential;
use credentials::credential_services::CredentialService;
use std::sync::Arc;
use users::user::User;
use users::user_services::UserService;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterRequest<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
    pub private_key: &'a str,
    pub public_key: &'a str,
}
impl RegisterRequest<'_> {
    async fn to_user(&self) -> anyhow::Result<User> {
        User::new(
            self.username.to_string(),
            self.email.to_string(),
            self.password.to_string(),
        )
    }

    async fn validate(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterResponse {}

pub struct RegisterUseCase {
    user_service: Arc<UserService>,
    credential_service: Arc<CredentialService>,
}

impl RegisterUseCase {
    pub fn new(user_service: Arc<UserService>, credential_service: Arc<CredentialService>) -> Self {
        Self {
            user_service,
            credential_service,
        }
    }
}

#[async_trait]
pub trait RegisterUseCaseInterface {
    async fn register<'a>(&self, request: &RegisterRequest<'a>)
        -> anyhow::Result<RegisterResponse>;
}

#[async_trait]
impl RegisterUseCaseInterface for RegisterUseCase {
    async fn register<'a>(
        &self,
        request: &RegisterRequest<'a>,
    ) -> anyhow::Result<RegisterResponse> {
        request.validate().await?;

        let user = request.to_user().await?;
        self.user_service.create_user(&user).await?;

        let credential = Credential::new(user.id, request.private_key, request.public_key);
        self.credential_service
            .create_credential(&credential)
            .await?;

        Ok(RegisterResponse {})
    }
}
