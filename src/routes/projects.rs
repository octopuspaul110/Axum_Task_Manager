

// Get /projects
use axum::{Json, extract::{Path, State}, http::StatusCode};
use uuid::Uuid;
use crate::{auth::middleware::AuthUser, error::AppError, models::project::{CreateProjectRequest, Project, UpdateProjectRequest}, state::AppState};

// GET/list_projects
pub async fn list_projects(
    State(state) : State<AppState>,
    user : AuthUser,
) -> Result<Json<Vec<Project>>,AppError> {

    let projects = sqlx::query_as!(
        Project,
            r#"
            SELECT id, user_id, name, description, created_at, updated_at
            FROM projects
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        user.user_id
    )
    .fetch_all(&state.db)
    .await?;

    Ok(Json(projects))
}


// POST /projects
pub async fn create_project(
    State(state) : State<AppState>,
    user : AuthUser,
    Json(body) : Json<CreateProjectRequest>,
) -> Result<(StatusCode,Json<Project>),AppError> {

    if body.name.trim().is_empty() {
        return Err(AppError::BadRequest("Project name cannot be empty".into()));
    }

    let project = sqlx::query_as!(
        Project,
        r#"
            INSERT INTO projects (id, user_id, name, description)
            VALUES ($1, $2, $3, $4)
            RETURNING id, user_id, name, description, created_at, updated_at
        "#,
        Uuid::new_v4(),
        user.user_id,
        body.name.trim(),
        body.description
    )
    .fetch_one(&state.db)
    .await?;

    Ok((
        StatusCode::CREATED,Json(project)
    ))
}

// GET /projects/{id}

pub async fn get_project(
    State(state) : State<AppState>,
    user : AuthUser,
    Path(project_id) : Path<Uuid>,
) ->  Result<Json<Project>,AppError> {
    let project = sqlx::query_as!(
        Project,
        r#"
            SELECT id, user_id, name, description, created_at, updated_at
            FROM projects
            WHERE id = $1 AND user_id = $2
        "#,
        project_id,
        user.user_id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| {
        AppError::NotFound(
            format!("Project {} not found",project_id)
        )
    })?;

    Ok(Json(project))
}

// PUT /projects/{id}

pub async fn update_project(
    State(state) : State<AppState>,
    user : AuthUser,
    Path(project_id) : Path<Uuid>,
    Json(body) : Json<UpdateProjectRequest>
) -> Result<Json<Project>,AppError> {

    let existing = sqlx::query_as!(
        Project,
        r#"
            SELECT id,user_id,name,description,created_at,updated_at
            FROM projects
            WHERE id = $1 AND user_id = $2
        "#,
        project_id,
        user.user_id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| {
        AppError::NotFound(
        format!("Project not found : {}",project_id)
        )
    })?;

    let new_name = body.name
    .as_deref()
    .map(str::trim)
    .filter(|n| {!n.is_empty()})
    .map(String::from)
    .unwrap_or(existing.name);

    let new_description = match body.description {
        Some(desc) => Some(desc),
        None => existing.description,
    };

    let project = sqlx::query_as!(
        Project,
        r#"
            UPDATE projects
            SET name = $1, description = $2, updated_at = NOW()
            WHERE id = $3 AND user_id = $4
            RETURNING id, user_id, name, description, created_at, updated_at
        "#,
        new_name,
        new_description,
        project_id,
        user.user_id
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(project))
}

// DELETE/projects/{id}
pub async fn delete_project(
    State(state) : State<AppState>,
    user : AuthUser,
    project_id : Uuid
)  -> Result<StatusCode,AppError>{


    let result = sqlx::query!(
        "DELETE FROM projects WHERE id = $1 and user_id = $2",
        project_id,
        user.user_id
    )
    .execute(&state.db)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(
            format!("Project {} not found", project_id)
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}