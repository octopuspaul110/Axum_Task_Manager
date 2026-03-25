use chrono::{DateTime, Utc};
use serde::{Serialize,Deserialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct Project {
    pub id : Uuid,
    pub user_id : Uuid,
    pub name : String,
    pub description : Option<String>,       // this can be null
    pub created_at : DateTime<Utc>,
    pub updated_at : DateTime<Utc>,
}
#[derive(Debug,Deserialize)]
pub struct CreateProjectRequest {
    pub name : String,
    pub description : Option<String>,
}

#[derive(Debug,Deserialize)]
pub struct UpdateProjectRequest {
    pub name : Option<String>,
    pub description : Option<String>
}

