use serde::Serialize;
use thiserror::Error;

pub const UNKNOWN_ERROR: u32 = 500;
pub const INVALID_INPUT: u32 = 400;
pub const UNAUTHORIZED: u32 = 401;

#[derive(Debug, Serialize, Error)]
pub enum GenericError {
    #[error("invalid input: {0}")]
    InvalidInput(String, u32),

    #[error("Login failed, No user and password found")]
    LoginFailed(u32),

    #[error("Something went wrong")]
    Unknown(),
}

impl GenericError {
    pub fn invalid_input(message: String) -> anyhow::Error {
        GenericError::InvalidInput(message, INVALID_INPUT).into()
    }

    pub fn login_failed() -> anyhow::Error {
        GenericError::LoginFailed(UNAUTHORIZED).into()
    }

    pub fn unknown() -> anyhow::Error {
        GenericError::Unknown().into()
    }
}
