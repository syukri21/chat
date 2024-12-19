use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Serialize, Error)]
pub enum GenericError<'a> {
    #[error("invalid input: {0}")]
    InvalidInput(&'a str, u32),

    #[error("Something went wrong")]
    Unknown(),
}

impl<'a> GenericError<'a> {
    pub fn invalid_input(message: &'a str) -> Self {
        GenericError::InvalidInput(message, 400)
    }
}
