use crate::domain::entity::task::{Task, TaskBody, TaskId, TaskStatus};
use crate::domain::error::DomainError;
use crate::domain::repository::task_repository::TaskRepository;
use async_trait::async_trait;
use futures_util::{StreamExt, TryStreamExt};
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

    async fn find_all(&self) -> Result<Vec<Task>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalTaskRepository::find_all(&mut conn).await
    }

    async fn create(&self, task: &Task) -> Result<(), DomainError> {
        let mut tx = self.pool.begin().await?;
        InternalTaskRepository::create(task, &mut tx).await?;
        tx.commit().await?;

        Ok(())
    }

    async fn update(&self, task: &Task) -> Result<(), DomainError> {
        let mut tx = self.pool.begin().await?;
        InternalTaskRepository::update(task, &mut tx).await?;
        tx.commit().await?;

        Ok(())
    }

    async fn delete(&self, id: &TaskId) -> Result<bool, DomainError> {
        todo!()
    }
}

pub struct InternalTaskRepository {}

impl InternalTaskRepository {
    pub async fn create(task: &Task, conn: &mut PgConnection) -> Result<(), DomainError> {
        sqlx::query("INSERT INTO task (id, body, status) VALUES ($1, $2, $3)")
            .bind(task.id.as_str())
            .bind(task.body.as_str())
            .bind(task.status.as_str())
            .execute(&mut *conn)
            .await?;

        Ok(())
    }

    async fn find_all(conn: &mut PgConnection) -> Result<Vec<Task>, DomainError> {
        let tasks = sqlx::query_as("SELECT * FROM task ORDER BY 1")
            .fetch(conn)
            .map(
                |row: Result<TaskRow, sqlx::Error>| -> Result<Task, DomainError> {
                    let row = row?;
                    let id = TaskId::new(row.id)?;
                    let body = TaskBody::new(row.body)?;
                    let status = TaskStatus::new(row.status)?;

                    let task = Task::new(id, body, status);
                    Ok(task)
                },
            )
            .try_collect()
            .await;

        tasks
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
        })
        .transpose()
    }

    async fn update(task: &Task, conn: &mut PgConnection) -> Result<(), DomainError> {
        sqlx::query("UPDATE task SET body = $1, status = $2 WHERE id = $3")
            .bind(task.body.as_str())
            .bind(task.status.as_str())
            .bind(task.id.as_str())
            .execute(&mut *conn)
            .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::postgres::PgPoolOptions;
    use sqlx::{Postgres, Transaction};
    use std::env::VarError;

    #[tokio::test]
    async fn test_create_and_find_by_id() -> anyhow::Result<()> {
        let mut tx = get_transaction().await?;

        let task = build_base_task().await?;

        let fetched_task = InternalTaskRepository::find_by_id(&task.id, &mut tx).await?;
        assert!(fetched_task.is_none());

        // create
        InternalTaskRepository::create(&task, &mut tx).await?;

        let fetched_task = InternalTaskRepository::find_by_id(&task.id, &mut tx).await?;
        assert_eq!(fetched_task, Some(task));

        tx.rollback().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_update_and_find_by_id() -> anyhow::Result<()> {
        let mut tx = get_transaction().await?;
        let task = build_base_task().await?;
        InternalTaskRepository::create(&task, &mut tx).await?;

        let new_body = TaskBody::new(String::from("new body"))?;
        let new_task = Task::new(
            task.id.clone(),
            TaskBody::new(String::from("new body"))?,
            TaskStatus::new(String::from("02"))?,
        );

        InternalTaskRepository::update(&new_task, &mut tx).await?;

        let fetched_task = InternalTaskRepository::find_by_id(&task.id, &mut tx).await?;
        assert_eq!(fetched_task, Some(new_task));

        tx.rollback().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_find_all() -> anyhow::Result<()> {
        let mut tx = get_transaction().await?;
        let task = build_base_task().await?;
        InternalTaskRepository::create(&task, &mut tx).await?;

        let fetched_task = InternalTaskRepository::find_all(&mut tx).await?;
        assert_eq!(fetched_task.len(), 1);

        tx.rollback().await?;

        Ok(())
    }

    async fn build_base_task() -> Result<Task, DomainError> {
        let task_id = TaskId::new(String::from("task 1"))?;
        let task_body = TaskBody::new(String::from("description"))?;
        let task_status = TaskStatus::new(String::from("05"))?;
        let task = Task::new(task_id.clone(), task_body, task_status);
        Ok(task)
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

    async fn get_transaction() -> Result<Transaction<'static, Postgres>, DomainError> {
        dotenv::dotenv().ok();

        let database_url = fetch_database_url();
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await?;

        Ok(pool.begin().await?)
    }
}
