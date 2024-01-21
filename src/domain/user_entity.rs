use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, sqlx::FromRow, Serialize, Clone)]
pub struct UserEntity {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub photo: String,
    pub verified: bool,

    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,

    #[serde(rename = "updatedAt")]
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct RegisterUserEntity {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserEntity {
    pub email: String,
    pub password: String,
}

