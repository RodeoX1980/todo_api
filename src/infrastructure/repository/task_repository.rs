use std::mem::transmute;

use crate::domain::entity::task::{Task, TaskBody, TaskId, TaskStatus};
use crate::domain::error::DomainError;
use crate::domain::repository::task_repository::TaskRepository;
use async_trait::async_trait;
use sqlx::{FromRow, PgConnection, PgPool};

#[derive(FromRow)]
struct TaskRow {
    id: String,
    body: String,
    status: String,
}

#[derive(Debug, Clone)]
pub struct PgTaskRepository {
    pool: PgPool,
}

impl PgTaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for PgTaskRepository {
    async fn find_by_id(&self, id: &TaskId) -> Result<Option<Task>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalTaskRepository::find_by_id(id, &mut conn).await
    }

    async fn create(&self, task: &Task) -> Result<(), DomainError> {
        let mut tx = self.pool.begin().await?;
        InternalTaskRepository::create(task, &mut tx).await?;
        tx.commit().await?;

        Ok(())
    }

    async fn update(&self, task: &Task) -> Result<(), DomainError> {
        todo!()
    }

    async fn delete(&self, id: &TaskId) -> Result<bool, DomainError> {
        todo!()
    }
}

pub(in crate::infrastructure) struct InternalTaskRepository {}

impl InternalTaskRepository {
    pub(in crate::infrastructure) async fn create(
        task: &Task,
        conn: &PgConnection,
    ) -> Result<(), DomainError> {
        sqlx::query("INSERT INTO task (id, body, status) VALUES ($1, $2, $3)")
            .bind(task.id.as_str())
            .bind(task.body.as_str())
            .bind(task.status.as_str())
            .execute(conn)
            .await?;

        Ok(())
    }

    async fn find_by_id(id: &TaskId, conn: &mut PgConnection) -> Result<Option<Task>, DomainError> {
        let row: Option<TaskRow> = sqlx::query_as("SELECT * FROM task WHERE id = $1")
            .bind(id.as_str())
            .execute(conn)
            .await?;

        let task = row.map(|row| {
            let id = TaskId::new(row.id)?;
            let body = TaskBody::new(row.body)?;
            let status = TaskStatus::new(row.status)?;

            Task::new { id, body, status }
        });
        let task = task.transpose()?;
        Ok(task)
    }
}
