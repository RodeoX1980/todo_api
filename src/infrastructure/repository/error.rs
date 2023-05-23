use sqlx::Error;
use crate::domain::error::DomainError;

impl From<Error> for DomainError {
    fn from(error: Error) -> Self {
        DomainError::InfrastructureError(anyhow::Error::new(error))
    }
}