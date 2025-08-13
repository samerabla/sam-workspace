use std::{cell::RefCell, collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};
use serde_json::to_value;
use time::OffsetDateTime;

use crate::AttributeMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "user_role", rename_all = "snake_case")
)]
pub enum UserRole {
    SuperAdmin,
    Admin,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub email: String,
    pub password: String,
    pub role: UserRole,
    pub attributes: AttributeMap,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub email: String,
    pub role: UserRole,
    pub attributes: AttributeMap,
    pub created_at: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HashUser {
    pub id: uuid::Uuid,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Default)]
pub struct UserState {
    pub email: Option<String>,
    pub redirect_to: Option<String>,
    pub loading: bool
}

impl UserState {
    pub fn new(email: Option<String>) -> Self {
        Self {
            email,
            redirect_to: None,
            loading: true,
        }
    }

   pub fn is_logged_in(&self) -> bool {
        !self.loading && self.email.is_some()
    }

    pub fn is_loading(&self) -> bool {
        self.loading
    }
}

pub type SharedUserState = Rc<RefCell<UserState>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub success: bool,
    pub data: UserResponseData,
    pub status_code: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserResponseData {
    String(String),
    Json(serde_json::Value),
}

impl UserResponse {
    pub fn new(success: bool, data: UserResponseData, status_code: u16) -> Self {
        Self {
            success,
            data,
            status_code,
        }
    }

    pub fn with_json(data: impl Serialize) -> Self {
        let json = to_value(data);
        match json {
            Ok(value) => Self::new(true, UserResponseData::Json(value), 200),
            Err(e) => Self::with_error(e.to_string()),
        }
    }

    pub fn has_json(&self) -> bool {
        self.success && matches!(self.data, UserResponseData::Json(_))
    }

    pub fn with_json_and_code(data: impl Serialize, status_code: u16) -> Self {
        let json = to_value(data);
        match json {
            Ok(value) => Self::new(true, UserResponseData::Json(value), status_code),
            Err(e) => Self::with_error(e.to_string()),
        }
    }

    pub fn with_success(msg: impl Into<String>) -> Self {
        Self::new(true, UserResponseData::String(msg.into()), 200)
    }

    pub fn with_success_and_code(msg: impl Into<String>, status_code: u16) -> Self {
        Self::new(true, UserResponseData::String(msg.into()), status_code)
    }

    pub fn with_error(msg: impl Into<String>) -> Self {
        Self::new(false, UserResponseData::String(msg.into()), 500)
    }

    pub fn with_error_and_code(msg: impl Into<String>, status_code: u16) -> Self {
        Self::new(false, UserResponseData::String(msg.into()), status_code)
    }

    pub fn message(&self) -> String {
        match &self.data {
            UserResponseData::String(msg) => msg.clone(),
            UserResponseData::Json(json) => json.to_string(),
        }
    }

    pub fn json(&self) -> Option<serde_json::Value> {
        match &self.data {
            UserResponseData::Json(json) => Some(json.clone()),
            UserResponseData::String(_) => None,
        }
    }
}
