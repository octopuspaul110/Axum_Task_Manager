use chrono::{DateTime, Utc};
use serde::{Serialize,Deserialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug,Clone,Serialize,Deserialize,Type,PartialEq)]
#[sqlx(type_name = "task_status",rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug,Clone,Serialize,Deserialize,Type,PartialEq)]
#[sqlx(type_name = "task_priority",rename_all = "snake_case")]
pub enum TaskPriority {
    Low,
    Medium,
    High
}

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct Task {
    pub id : Uuid,
    pub project_id : Uuid,
    pub title : String,
    pub description : Option<String>,
    pub status : TaskStatus,
    pub priority : TaskPriority,
    pub due_date : Option<DateTime<Utc>>,
    pub created_at : DateTime<Utc>,
    pub updated_at : DateTime<Utc>
}

#[derive(Debug,Deserialize)]
pub struct CreateTaskRequest {
    pub title : String,
    pub description : Option<String>,
    pub priority : Option<TaskPriority>,
    pub due_date : Option<DateTime<Utc>>
}

#[derive(Debug,Deserialize)]
pub struct UpdateTaskRequest {
    pub title : Option<String>,
    pub description : Option<String>,
    pub status : Option<TaskStatus>,
    pub priority : Option<TaskPriority>,
    pub due_date : Option<DateTime<Utc>>
}