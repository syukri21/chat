use async_trait::async_trait;
use commons::generic_errors::GenericError;
use credentials::credential::Credential;
use credentials::credential_services::CredentialService;
use mail::SendEmail;
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
        // check if email is valid
        let email_err = "Email is not valid";
        if !self.email.contains('@') || !self.email.contains('.') {
            return Err(GenericError::invalid_input(email_err).into());
        }

        let password_err = "Password must be at least 8 characters and contain at least one number";
        if self.password.len() < 8 || !self.password.chars().any(char::is_numeric) {
            return Err(GenericError::invalid_input(password_err).into());
        }

        let username_err = "Username must be at least 3 characters";
        if self.username.len() < 3 {
            return Err(GenericError::invalid_input(username_err).into());
        }

        let public_key_err = "Public key is empty";
        if self.public_key.is_empty() {
            return Err(GenericError::invalid_input(public_key_err).into());
        }

        let private_key_err = "Private key is empty";
        if self.private_key.is_empty() {
            return Err(GenericError::invalid_input(private_key_err).into());
        }

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegisterResponse {}

pub struct RegisterUseCase {
    user_service: Arc<UserService>,
    credential_service: Arc<CredentialService>,
    mail: Arc<dyn SendEmail + Send + Sync>,
}

impl RegisterUseCase {
    pub fn new(
        user_service: Arc<UserService>,
        credential_service: Arc<CredentialService>,
        mail: Arc<dyn SendEmail + Send + Sync>,
    ) -> Self {
        Self {
            user_service,
            credential_service,
            mail,
        }
    }
}

#[async_trait]
pub trait RegisterUseCaseInterface {
    async fn register<'a>(&self, request: &RegisterRequest<'a>)
        -> anyhow::Result<RegisterResponse>;
    async fn activate_user<'a>(&self, encrypted_user_id: &'a str) -> anyhow::Result<()>;
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

        self.mail.send_email(
            &user.username,
            &user.email,
            "Registration successful",
            "Your account has been created successfully, Please activate your account. Click the link below to activate your account, <a href=\"https://example.com/activate/123\">Activate</a>",
        ).await?;

        Ok(RegisterResponse {})
    }

    async fn activate_user<'a>(&self, _encrypted_user_id: &'a str) -> anyhow::Result<()> {
        // TODO: add encryption crate
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_register_validate_should_fail_with_invalid_input_email() {
        let request = RegisterRequest {
            username: "test",
            email: "test@examplecom",
            password: "password1",
            private_key: "privatekey",
            public_key: "publickey",
        };

        assert_invalid_input_error(request.validate().await, "email").await;
    }

    #[tokio::test]
    async fn test_register_validate_should_fail_with_invalid_input_password() {
        let request = RegisterRequest {
            username: "test",
            email: "test@example.com",
            password: "test",
            private_key: "",
            public_key: "",
        };

        assert_invalid_input_error(request.validate().await, "password").await;
    }

    #[tokio::test]
    async fn test_register_validate_should_fail_with_invalid_input_username() {
        let request = RegisterRequest {
            username: "te",
            email: "test@example.com",
            password: "password1",
            private_key: "privatekey",
            public_key: "publickey",
        };

        assert_invalid_input_error(request.validate().await, "username").await;
    }

    #[tokio::test]
    async fn test_register_validate_should_fail_with_empty_public_key() {
        let request = RegisterRequest {
            username: "testuser",
            email: "test@example.com",
            password: "password1",
            private_key: "privatekey",
            public_key: "",
        };

        assert_invalid_input_error(request.validate().await, "public key").await;
    }

    #[tokio::test]
    async fn test_register_validate_should_fail_with_empty_private_key() {
        let request = RegisterRequest {
            username: "testuser",
            email: "test@example.com",
            password: "password1",
            private_key: "",
            public_key: "publickey",
        };

        assert_invalid_input_error(request.validate().await, "private key").await;
    }

    async fn assert_invalid_input_error(result: anyhow::Result<()>, expected_message: &str) {
        let error = result.unwrap_err();
        match error.downcast::<GenericError>().unwrap() {
            GenericError::InvalidInput(message, 400) => {
                assert!(message
                    .to_string()
                    .to_lowercase()
                    .contains(expected_message));
            }
            _ => assert!(false),
        }
    }
}
