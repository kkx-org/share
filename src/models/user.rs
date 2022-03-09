use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Option<Uuid>,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserDto {
    pub username: Option<String>,
    pub email: Option<String>,
}
