use crate::domain::repository::task_repository::TaskRepository;

pub struct UseCase<R: TaskRepository> {
    repository: R,
}

impl<R: TaskRepository> UseCase<R> {
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
}
