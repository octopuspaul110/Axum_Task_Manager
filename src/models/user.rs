
use serde::{Serialize,Deserialize};
use sqlx::FromRow;
use uuid::Uuid as Uuid;
use chrono::{DateTime,Utc};

#[derive(Debug,Clone,Serialize,Deserialize,FromRow)]
pub struct User {
    pub id: Uuid,
    pub email : String,
    pub name : String,

    #[serde(skip_serializing)]
    pub password_hash : String,

    pub created_at : DateTime<Utc>,
    pub updated_at : DateTime<Utc>
}
#[derive(Debug,Serialize)]
pub struct UserResponse {
    pub id : Uuid,
    pub email : String,
    pub name : String,
    pub created_at : DateTime<Utc>
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse { 
            id: user.id, 
            email: user.email, 
            name: user.name, 
            created_at: user.created_at
        }
    }
}