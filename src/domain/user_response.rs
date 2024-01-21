use chrono::prelude::*;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct FilteredUser {
    pub id: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub photo: String,
    pub verified: bool,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserData {
    pub user: FilteredUser,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub status: String,
    pub data: UserData,
}
