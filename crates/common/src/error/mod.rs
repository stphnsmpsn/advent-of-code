use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("`{0}`")]
    OverflowError(String),
    #[error("Invalid Input: `{0}`")]
    InputError(String),
    #[error("Error: `{0}`")]
    GenericError(String),
}
