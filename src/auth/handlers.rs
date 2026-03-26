use axum::{Json, extract::State, http::StatusCode};
use bcrypt::{DEFAULT_COST, hash};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::jwt::create_access_token, error::AppError, models::user::{User, UserResponse}, state::AppState};

#[derive(Debug,Deserialize)]
pub struct RegisterRequest {
    pub name : String,
    pub email : String,
    pub password : String,
}


#[derive(Debug,Serialize)]
pub struct AuthResponse {
    pub token : String,
    pub user : UserResponse,
}

pub async fn register(
    State(state) : State<AppState>,
    Json(body) : Json<RegisterRequest>
) -> Result<(StatusCode,Json<AuthResponse>),AppError> {

    if body.name.trim().is_empty() {
        return Err(AppError::BadRequest("Name cannot be empty".into()));
    }
    if body.email.trim().is_empty() || !body.email.trim().contains("@") {
        return Err(AppError::BadRequest("Provide a valid email address".into()));
    }
    if body.password.len() < 8 {
        return Err(AppError::BadRequest("password must be at least 8 character long.".into()));
    }

    let existing = sqlx::query_scalar!(
        "SELECT id FROM users WHERE email = $1",
        body.email.to_lowercase()
    ).fetch_optional(&state.db)
    .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("An account with this email address already exists".into()));
    }

    let password_hash = hash(&body.password, DEFAULT_COST)
    .map_err(|e|{
        tracing::error!("bcrypt hashing failed : {:?}",e);
        AppError::Internal
    })?;

    let user = sqlx::query_as!(
        User,
        r#"
            INSERT INTO users(id,email,name,password_hash)
            VALUES ($1,$2,$3,$4)
            RETURNING id, email, name, password_hash, created_at, updated_at
        "#,
        Uuid::new_v4(),
        body.email.to_lowercase(),
        body.name.trim(),
        password_hash
    )
    .fetch_one(&state.db)
    .await?;

    let token = create_access_token(user.id, &user.email, &state.jwt_secret)
    .map_err(|e|{
        tracing::error!("JWT creation failed: {:?}",e);
        AppError::Internal
    })?;
    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            user : user.into()
        })
    ))
}

