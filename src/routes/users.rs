use axum::{Json, extract::State};
use crate::{auth::middleware::AuthUser, error::AppError, models::user::UserResponse, state::AppState};

pub async fn get_me(
    State(state): State<AppState>,
    user : AuthUser
) -> Result<Json<UserResponse>,AppError> {
    let db_user = sqlx::query_as!(
        crate::models::user::User,
        r#"
            SELECT id, email, name, password_hash, created_at, updated_at
            FROM users
            WHERE id = $1
        "#,
        user.user_id
    )
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(||{
        AppError::NotFound("user not found".into()
        )
    })?;

    Ok(Json(db_user.into()))
} 