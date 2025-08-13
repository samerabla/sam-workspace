use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
pub use shared::user::UserResponse;

struct UserResponseWrapper(UserResponse);

impl IntoResponse for UserResponseWrapper {
    fn into_response(self) -> Response {
        let status_code = match StatusCode::from_u16(self.0.status_code) {
            Ok(s) => s,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, Json(self.0)).into_response()
    }
}

pub trait IntoUserResponse {
    fn into_response(self) -> Response;
}

impl IntoUserResponse for UserResponse {
    fn into_response(self) -> Response {
        UserResponseWrapper(self).into_response()
    }
}
