use anyhow::Error;
use log::error;
use serde::Serialize;
use thiserror::Error;

pub const UNKNOWN_ERROR: u32 = 500;
pub const INVALID_INPUT: u32 = 400;
pub const UNAUTHORIZED: u32 = 401;

#[derive(Debug, Serialize, Error)]
pub enum GenericError {
    #[error("{0}")]
    InvalidInput(String, u32),

    #[error("Login failed, No user and password found")]
    LoginFailed(u32),

    #[error("Token is invalid")]
    InvalidToken(u32),

    #[error("Token is expired")]
    TokenExpired(u32),

    #[error("Something went wrong")]
    Unknown(),

    #[error("User not found")]
    UserNotFound(),
}

impl GenericError {
    pub fn invalid_input(message: String) -> anyhow::Error {
        error!("invalid input: {}", message);
        GenericError::InvalidInput(message, INVALID_INPUT).into()
    }

    pub fn login_failed() -> anyhow::Error {
        error!("login failed");
        GenericError::LoginFailed(UNAUTHORIZED).into()
    }

    pub fn unknown(e: anyhow::Error) -> anyhow::Error {
        error!("error: {}", e);
        GenericError::Unknown().into()
    }

    pub fn invalid_token() -> anyhow::Error {
        error!("invalid token");
        GenericError::InvalidToken(UNAUTHORIZED).into()
    }

    pub fn token_expired() -> anyhow::Error {
        error!("token expired");
        GenericError::TokenExpired(UNAUTHORIZED).into()
    }

    pub fn user_not_found(error: Error) -> anyhow::Error {
        error!("user not found :{}", error);
        GenericError::UserNotFound().into()
    }
}
