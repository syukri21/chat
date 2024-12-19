use derive_more::{Display, From};

#[derive(Debug, From, Display)]
pub enum GenericError {
    #[display(fmt = "Invalid input: {0}")]
    #[from]
    InvalidInput(String),

    #[from]
    #[display(fmt = "Database error: {0}")]
    DatabaseError(String),

    #[from]
    #[display(fmt = "Something went wrong: {0}")]
    Unknown(String),
}
