use axum::http::header;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Serialize,Deserialize};
use uuid::Uuid;




#[derive(Debug,Serialize,Deserialize)]
pub struct Claims {
    pub sub : String,
    pub email : String,
    pub exp : i64,
    pub iat : i64
}

pub fn create_access_token(
    user_id : Uuid,
    email : &str,
    secret : &str
) -> Result<String,jsonwebtoken::errors::Error> {
    let now = Utc::now();

    let claims = Claims{
        sub : user_id.to_string(),
        email : email.to_string(),
        exp:(now + Duration::hours(24)).timestamp(),
        iat : now.timestamp()
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}

pub fn verify_token(
    token : &str,
    secret : &str
) -> Result<Claims,jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}


