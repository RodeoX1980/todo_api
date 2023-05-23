use async_trait::async_trait;
use crate::domain::entity::task::{Task, TaskBody, TaskId, TaskStatus};
use crate::domain::error::DomainError;
use crate::domain::repository::task_repository::TaskRepository;
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

pub struct InternalTaskRepository {}

impl InternalTaskRepository {
    pub async fn create(
        task: &Task,
        conn: &mut PgConnection,
    ) -> Result<(), DomainError> {
        sqlx::query("INSERT INTO task (id, body, status) VALUES ($1, $2, $3)")
            .bind(task.id.as_str())
            .bind(task.body.as_str())
            .bind(task.status.as_str())
            .execute(&mut *conn)
            .await?;

        Ok(())
    }

    async fn find_by_id(id: &TaskId, conn: &mut PgConnection) -> Result<Option<Task>, DomainError> {
        let row: Option<TaskRow> = sqlx::query_as("SELECT * FROM task WHERE id = $1")
            .bind(id.as_str())
            .fetch_optional(conn)
            .await?;

        row.map(|row| -> Result<Task, DomainError> {
            let id = TaskId::new(row.id)?;
            let body = TaskBody::new(row.body)?;
            let status = TaskStatus::new(row.status)?;
            Ok(Task::new(id, body, status))
        }).transpose()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use std::env::VarError;

    #[tokio::test]
    async fn test_create_and_find_by_id() -> anyhow::Result<()> {
        dotenv::dotenv().ok();

        let database_url = fetch_database_url();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        let mut tx = pool.begin().await?;

        let task_id = TaskId::new(String::from("task 1"))?;
        let task_body = TaskBody::new(String::from("description"))?;
        let task_status = TaskStatus::new(String::from("05"))?;
        let task = Task::new(task_id.clone(), task_body, task_status);

        let fetched_task = InternalTaskRepository::find_by_id(&task_id, &mut tx).await?;
        assert!(fetched_task.is_none());

        // create
        InternalTaskRepository::create(&task, &mut tx).await?;

        let fetched_task = InternalTaskRepository::find_by_id(&task_id, &mut tx).await?;
        assert_eq!(fetched_task, Some(task));

        tx.rollback();
        Ok(())
    }

    fn fetch_database_url() -> String {
        match std::env::var("DATABASE_URL") {
            Ok(s) => s,
            Err(VarError::NotPresent) => panic!("Environment variable DATABASE_URL is required."),
            Err(VarError::NotUnicode(_)) => {
                panic!("Environment variable DATABASE_URL is not unicode.")
            }
        }
    }
}
