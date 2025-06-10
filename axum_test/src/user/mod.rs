use crate::error::Result;
use password::hash_password;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use time::OffsetDateTime;

pub use auth::auth_middleware;
pub use user_routes::user_routes;

//delete later
pub use jwt::Claims;

mod auth;
mod cookie;
mod jwt;
mod password;
mod user_db;
mod user_emails;
mod user_routes;

#[derive(Debug, Clone, Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

impl LoginUser {
    pub fn hash(mut self) -> Result<Self> {
        self.password = hash_password(self.password.as_str())?;
        Ok(self)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

#[derive(Debug, FromRow)]
pub struct HashUser {
    id: uuid::Uuid,
    password: String,
    email: String,
}

#[derive(Debug, FromRow)]
pub struct UserInfo {
    id: String,
    email: String,
    created_at: OffsetDateTime,
}

#[derive(Debug, FromRow)]
pub struct User {
    id: String,
    email: String,
    password: String,
    created_at: OffsetDateTime,
}
