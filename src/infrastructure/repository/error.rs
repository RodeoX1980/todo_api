use crate::domain::error::DomainError;
use sqlx::Error;

impl From<Error> for DomainError {
    fn from(error: Error) -> Self {
        DomainError::InfrastructureError(anyhow::Error::new(error))
    }
}
