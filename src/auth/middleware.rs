use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use uuid::Uuid;
use crate::{auth::jwt::verify_token, error::AppError, state::AppState};

#[derive(Debug,Clone)]
pub struct AuthUser {
    pub user_id : Uuid,
    pub email : String,
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self,Self::Rejection> {

        let auth_header = parts
        .headers
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

        let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

        let claims = verify_token(token, &state.jwt_secret)
        .map_err(|_| AppError::Unauthorized)?;

        let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized)?;

        Ok(
            AuthUser {
            user_id,
            email : claims.email
        })   
    }
}