use crate::domain::error::DomainError;
use validator::Validate;

#[derive(Debug, PartialEq, Eq, Clone, Validate)]
pub struct TaskId {
    value: String,
}

impl TaskId {
    pub fn new(id: String) -> Result<Self, DomainError> {
        let object = Self { value: id };
        object.validate()?;
        Ok(object)
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn into_string(self) -> String {
        self.value
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Validate)]
pub struct TaskBody {
    value: String,
}

impl TaskBody {
    pub fn new(body: String) -> Result<Self, DomainError> {
        let object = Self { value: body };
        object.validate()?;
        Ok(object)
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn into_string(self) -> String {
        self.value
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Validate)]
pub struct TaskStatus {
    #[validate(length(max = 2))]
    value: String,
}

impl TaskStatus {
    pub fn new(status: String) -> Result<Self, DomainError> {
        let object = Self { value: status };
        object.validate()?;
        Ok(object)
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn into_string(self) -> String {
        self.value
    }
}

pub struct Task {
    pub id: TaskId,
    pub body: TaskBody,
    pub status: TaskStatus,
}

impl Task {
    pub fn new(id: TaskId, body: TaskBody, status: TaskStatus) -> Self {
        Self { id, body, status }
    }
}
