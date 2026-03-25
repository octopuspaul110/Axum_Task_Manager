use std::sync::Arc;

use sqlx::PgPool;



#[derive(Clone)]
pub struct AppState {
    pub db : PgPool,
    pub jwt_secret : Arc<String>
}