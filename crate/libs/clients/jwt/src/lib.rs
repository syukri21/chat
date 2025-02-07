use async_trait::async_trait;
use chrono::Duration;
use commons::generic_errors::GenericError;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use persistence::env::myenv::EnvInterface;
use serde::{Deserialize, Serialize};
use shaku::{Component, Interface};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

const ROLE_ADMIN: &str = "admin";
const ROLE_USER: &str = "user";

const DEFAULT_EXP_IN_SECONDS: i64 = 24 * 60 * 60; // 24 hours
const DEFAULT_JWT_ALG: &str = "HS256";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessClaims {
    pub exp: i64,
    pub user_id: String,
    pub role: Role,
    pub permissions: Vec<String>,
    pub jti: String,
    pub alg: String,
}
impl AccessClaims {
    pub fn new(user_id: String, role: Role) -> Self {
        Self::new_with_exp(user_id, role, generate_exp())
    }
    fn new_with_exp(user_id: String, role: Role, exp: i64) -> Self {
        AccessClaims {
            exp,
            user_id,
            role,
            permissions: vec![],
            jti: generate_jti(),
            alg: DEFAULT_JWT_ALG.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JWTToken {
    pub token: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[derive(Default)]
pub enum Role {
    Admin,
    #[default]
    User,
}

impl Display for Role {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Role::Admin => ROLE_ADMIN.to_string(),
            Role::User => ROLE_USER.to_string(),
        };
        write!(f, "{}", str)
    }
}


impl From<&str> for Role {
    fn from(value: &str) -> Self {
        match value {
            ROLE_ADMIN => Role::Admin,
            _ => Role::User,
        }
    }
}

#[async_trait]
pub trait JWTInterface: Interface {
    async fn create_token(&self, user_id: &str, role: &str) -> anyhow::Result<JWTToken>;
    async fn generate_token(&self, claims: &AccessClaims) -> anyhow::Result<JWTToken>;
    async fn verify_token(&self, token: &str) -> anyhow::Result<AccessClaims>;
}

#[derive(Component)]
#[shaku(interface = JWTInterface)]
pub struct JWT {
    #[shaku(inject)]
    env: Arc<dyn EnvInterface>,
}
#[async_trait]
impl JWTInterface for JWT {
    async fn create_token(&self, user_id: &str, role: &str) -> anyhow::Result<JWTToken> {
        let claims = AccessClaims::new(user_id.to_string(), role.into());
        self.generate_token(&claims).await
    }

    async fn generate_token(&self, claims: &AccessClaims) -> anyhow::Result<JWTToken> {
        let token = jsonwebtoken::encode(
            &jsonwebtoken::Header::new(Algorithm::HS256),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(self.env.get_app_key_jwt().as_ref()),
        )?;
        Ok(JWTToken { token })
    }

    async fn verify_token(&self, token: &str) -> anyhow::Result<AccessClaims> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_aud = false;

        let decoded = decode::<AccessClaims>(
            token,
            &DecodingKey::from_secret(self.env.get_app_key_jwt().as_ref()),
            &validation,
        );

        decoded
            .map_err(|err| match err.kind() {
                ErrorKind::ExpiredSignature => GenericError::token_expired(),
                _ => GenericError::invalid_token(),
            })
            .map(|token| token.claims)
    }
}

fn generate_exp() -> i64 {
    // Current time in UTC
    let now = chrono::Local::now();
    // Add the duration to current time
    let expiration_time = now + Duration::seconds(DEFAULT_EXP_IN_SECONDS);
    // Return the expiration time as a UNIX timestamp
    expiration_time.timestamp()
}

fn generate_jti() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use persistence::Env;
    use shaku::{module, HasComponent};

    module! {
        Module {
            components = [JWT, Env],
            providers = []
        }
    }

    #[test]
    fn test_generate_exp() {
        let exp = generate_exp();
        assert!(exp > 0);
    }

    #[test]
    fn test_generate_jti() {
        let jti = generate_jti();
        assert!(!jti.is_empty());
    }

    #[test]
    fn test_role_to_string() {
        let role = Role::Admin;
        assert_eq!(role.to_string(), ROLE_ADMIN);
    }

    #[test]
    fn test_role_from_str() {
        let role = Role::from(ROLE_ADMIN);
        assert_eq!(role, Role::Admin);
    }

    #[tokio::test]
    async fn test_create_token() {
        let module = Module::builder()
            .with_component_override::<dyn EnvInterface>(Box::new(Env::load()))
            .build();
        let jwt: &dyn JWTInterface = module.resolve_ref();
        let uuid1 = uuid::Uuid::new_v4();
        let token = jwt
            .create_token(uuid1.to_string().as_ref(), "admin")
            .await
            .unwrap();
        assert!(!token.token.is_empty());
    }

    #[tokio::test]
    async fn test_verify_token() {
        let module = Module::builder()
            .with_component_override::<dyn EnvInterface>(Box::new(Env::load()))
            .build();
        let jwt: &dyn JWTInterface = module.resolve_ref();
        let uuid1 = uuid::Uuid::new_v4();
        let token = jwt
            .create_token(uuid1.to_string().as_ref(), "admin")
            .await
            .unwrap();
        let claims = jwt.verify_token(&token.token).await.unwrap();
        assert_eq!(claims.user_id, uuid1.to_string());
        assert_eq!(claims.role, Role::Admin);
    }

    #[tokio::test]
    async fn test_verify_token_invalid_token() {
        let module = Module::builder()
            .with_component_override::<dyn EnvInterface>(Box::new(Env::load()))
            .build();
        let jwt: &dyn JWTInterface = module.resolve_ref();
        let result = jwt.verify_token("invalid_token").await;
        assert!(result.unwrap_err().to_string().contains("invalid"));
    }

    #[tokio::test]
    async fn test_verify_token_expired_token() {
        let module = Module::builder()
            .with_component_override::<dyn EnvInterface>(Box::new(Env::load()))
            .build();
        let jwt: &dyn JWTInterface = module.resolve_ref();
        let uuid1 = uuid::Uuid::new_v4();

        let now = chrono::Local::now();
        // Add the duration to current time
        let expiration_time = now - Duration::seconds(DEFAULT_EXP_IN_SECONDS);
        // Return the expiration time as a UNIX timestamp
        let exp = expiration_time.timestamp();

        let claims = AccessClaims::new_with_exp(uuid1.to_string(), Role::User, exp);
        let token = jwt.generate_token(&claims).await.unwrap();
        let result = jwt.verify_token(&token.token).await;
        assert!(result.unwrap_err().to_string().contains("expired"));
    }
}
