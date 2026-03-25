use axum::{Json, http::StatusCode, response::{IntoResponse,Response}};
use thiserror::Error;
use serde_json::json;

#[derive(Debug,Error)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Authentication required")]
    Unauthorized,

    #[error("Access denied: {0}")]
    Forbidden(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Internal server error")]
    Database(#[from] sqlx::Error),

    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        
        let (status,message,code) = match &self {
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,   //404
                msg.clone(),
                "NOT_FOUND"
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,   //401
                "Authentication required".to_string(),
                "UNAUTHORIZED"
            ),
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,      //403
                msg.clone(),
                "FORBIDDEN"
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,       //409
                msg.clone(),
                "CONFLICT"
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,    //400
                msg.clone(),
                "BAD_REQUEST"
            ),
            AppError::Database(db_err) => {
                tracing::error!("Databse error: {:?}",db_err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,      //500
                    "A database error occured".to_string(),
                    "DATABASE_ERROR",
                )
            },
            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,      //500
                "An unexpected error occured".to_string(),
                "INTERNAL_ERROR"
            ),
        };

        let body = Json(
            json!(
                {
                    "error" : {
                        "code" : code,
                        "message" : message
                    }
                }
            )
        );

        (status,body).into_response() //this is the important part
    }
}