use crate::domain::entity::task::{Task, TaskId};
use crate::domain::error::DomainError;
use async_trait::async_trait;
use mockall::automock;

#[automock]
#[async_trait]
pub trait TaskRepository: Send + Sync + 'static {
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<Task>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Task>, DomainError>;
    async fn create(&self, task: &Task) -> Result<(), DomainError>;
    async fn update(&self, task: &Task) -> Result<(), DomainError>;
    async fn delete(&self, id: &TaskId) -> Result<bool, DomainError>;
}
