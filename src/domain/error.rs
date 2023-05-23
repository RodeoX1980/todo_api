use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
pub enum DomainError {
    #[error("{0}")]
    Validation(String),
    #[error("ID {0} not found.")]
    NotFound(String),
    #[error(transparent)]
    Other(anyhow::Error),
}

impl From<ValidationErrors> for DomainError {
    fn from(error: ValidationErrors) -> Self {
        DomainError::Validation(error.to_string())
    }
}
