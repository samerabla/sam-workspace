use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
pub use frontend::models::UserResponse;
use serde::{Deserialize, Serialize};

pub trait IntoCustomResponse {
    fn into_success_response(self) -> Response;
    fn into_error_response(self) -> Response;
    fn into_gen_error_response() -> Response;
}

pub struct UserResponseWrapper(UserResponse);

impl IntoCustomResponse for UserResponse {
    fn into_success_response(mut self) -> Response {
        self.success = true;
        UserResponseWrapper(self).into_response()
    }

    fn into_error_response(mut self) -> Response {
        self.success = false;
        UserResponseWrapper(self).into_response()
    }

    /// Returns a response with a general error
    fn into_gen_error_response() -> Response {
        Self::new("Something went wrong", 500).into_error_response()
    }
}

impl IntoResponse for UserResponseWrapper {
    fn into_response(self) -> Response {
        let status_code = match StatusCode::from_u16(self.0.status_code) {
            Ok(s) => s,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, Json(self.0)).into_response()
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct UserResponse {
//     success: bool,
//     message: String,
//     status_code: u16,
//     data: Option<serde_json::Value>,
// }

// impl IntoCustomResponse for UserResponse {
//     fn into_success_response(mut self) -> Response {
//         self.success = true;
//         self.into_response()
//     }

//     fn into_error_response(mut self) -> Response {
//         self.success = false;
//         self.into_response()
//     }

//     // Used to send response with general error
//     fn some_error() -> Response {
//         Self::new("Something went wrong", 500).into_error_response()
//     }
// }

// impl UserResponse {
//     pub fn new(message: impl Into<String>, status_code: u16) -> Self {
//         Self {
//             success: true,
//             message: message.into(),
//             status_code,
//             data: None,
//         }
//     }

//     pub fn with_data(mut self, data: Option<serde_json::Value>) -> Self {
//         self.data = data;
//         self
//     }

//     pub fn into_success_response(mut self) -> Response {
//         self.success = true;
//         self.into_response()
//     }

//     pub fn into_error_response(mut self) -> Response {
//         self.success = false;
//         self.into_response()
//     }

//     // Used to send response with general error
//     pub fn some_error() -> Response {
//         Self::new("Something went wrong", 500).into_error_response()
//     }
// }

// impl IntoResponse for UserResponse {
//     fn into_response(self) -> Response {
//         let status_code = match StatusCode::from_u16(self.status_code) {
//             Ok(s) => s,
//             Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
//         };
//         (status_code, Json(self)).into_response()
//     }
// }
