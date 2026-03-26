
use axum::{Json, extract::{Path, State}, http::StatusCode};
use uuid::Uuid;

use crate::{auth::middleware::AuthUser, error::AppError, models::task::{CreateTaskRequest, Task, TaskStatus, TaskPriority}, state::AppState};

pub async fn create_task(
    State(state) : State<AppState>,
    user : AuthUser,
    Path(project_id) : Path<Uuid>,
    Json(body) : Json<CreateTaskRequest>,
) -> Result<(StatusCode,Json<Task>),AppError>{
    if body.title.trim().is_empty() {
        return Err(AppError::BadRequest("Task title cannot be empty".into()));
    }

    let project_exists = sqlx::query_scalar!(
        "SELECT id FROM projects WHERE id = $1 AND user_id = $2",
        project_id,
        user.user_id
    )
    .fetch_optional(&state.db)
    .await?;

    if project_exists.is_none() {
        return Err(AppError::NotFound(
            format!("Project {} not found",project_id)
        ));
    }

    let task = sqlx::query_as!(
        Task,
        r#"
            INSERT INTO tasks (id, project_id, title, description, priority, due_date)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING
                id,
                project_id,
                title,
                description,
                status AS "status:TaskStatus",
                priority AS "priority:TaskPriority",
                due_date,
                created_at,
                updated_at
        "#,
        Uuid::new_v4(),
        project_id,
        body.title.trim(),
        body.description,
        body.priority.unwrap_or(TaskPriority::Medium) as TaskPriority,
        body.due_date 
    )
    .fetch_one(&state.db)
    .await?;
    
    Ok((StatusCode::CREATED,Json(task)))

}


// GET/projects/{id}/tasks
pub async fn list_tasks(
    State(state) : State<AppState>,
    user : AuthUser,
    Path(project_id) : Path<Uuid>,
) -> Result<Json<Vec<Task>>,AppError>{

    let project_exists = sqlx::query_scalar!(
        "SELECT id FROM projects WHERE id = $1 AND user_id = $2",
        project_id,
        user.user_id
    )
    .fetch_optional(&state.db)
    .await?;

    if project_exists.is_none() {
        return Err(AppError::NotFound(
            format!("project does not exist: {}",project_id)
        ));
    }

    let tasks = sqlx::query_as!(
        Task,
        r#"
            SELECT
                id,
                project_id,
                title,
                description,
                status AS "status: TaskStatus",
                priority AS "priority: TaskPriority",
                due_date,
                created_at,
                updated_at
            FROM tasks
            WHERE project_id = $1
            ORDER BY created_at ASC
        "#,
        project_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(tasks))
}


// DELETE /tasks/{id}
pub async fn delete_task(
    State(state) : State<AppState>,
    user : AuthUser,
    Path(task_id) : Path<Uuid>
) -> Result<StatusCode,AppError> {

    let result = sqlx::query!(
        r#"
            DELETE FROM tasks
            WHERE id = $1
                AND project_id IN (
                    SELECT id FROM projects WHERE user_id = $2
                )
        "#,
        task_id,
        user.user_id
    )
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            format!("Task {} not found",task_id)
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}