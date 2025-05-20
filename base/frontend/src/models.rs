use std::{cell::RefCell, rc::Rc};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddUser {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, Default)]
pub struct UserState {
    pub loggedin: bool,
}

pub type SharedUserState = Rc<RefCell<UserState>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    pub success: bool,
    pub message: String,
    pub status_code: u16,
    pub data: Option<serde_json::Value>,
}

impl UserResponse {
    pub fn new(message: impl Into<String>, status_code: u16) -> Self {
        Self {
            success: true,
            message: message.into(),
            status_code,
            data: None,
        }
    }

    pub fn with_data(mut self, data: Option<serde_json::Value>) -> Self {
        self.data = data;
        self
    }
}
