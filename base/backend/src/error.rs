use sam_error::SamError;
pub type Result<T, E = SamError> = std::result::Result<T, E>;

use sqlx::{Error as SqlxError, PgPool};
use std::any::Any;
use tracing::error;

use axum::{
    body::to_bytes,
    extract::{rejection::JsonRejection, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
// use thiserror::Error;

use crate::{
    response::{IntoCustomResponse, UserResponse},
    AppState,
};
type BoxError = Box<dyn Any + Send + 'static>;

// #[derive(Error, Debug)]
// pub enum SamError {
//     #[error("{source}")]
//     Database {
//         #[source]
//         source: sqlx::Error,
//     },
//     #[error("{0}")]
//     Json(#[from] JsonRejection),
//     #[error("Login failed: email or password is wrong")]
//     LoginFailed,
//     #[error("Registration failed.")]
//     RegistrationFailed,
//     #[error("Not authorized.Please login")]
//     NotAuthorized,
//     #[error("Invalid token.")]
//     InvalidToken,
//     #[error("Expired token.")]
//     ExpiredToken(String),
//     #[error("Missing Enviroment Variable: {0}")]
//     MissingEnviromentVariable(String),
//     #[error("Email Send Failed.")]
//     EmailSendFailed,
//     #[error("Email Not Found")]
//     EmailNotFound,
//     #[error("Debug Error >>> {0}")]
//     Debug(String),
//     #[error("Something went wrong")]
//     Any,
// }

// // We implement From when we need to use more than one inputs for the enum variant but in this case
// // we should use source attribute instead from attribute
// impl From<sqlx::Error> for SamError {
//     fn from(source: sqlx::Error) -> Self {
//         SamError::Database { source }
//     }
// }

// // pub trait IntoErrorResponse {
// //     fn into_error_response(self) -> Response;
// // }

// // impl IntoErrorResponse for (StatusCode, SamError) {
// //     fn into_error_response(self) -> Response {
// //         let status_code = self.0.as_u16();
// //         UserResponse::new(self.1.to_string(), status_code).into_error_response()
// //     }
// // }

// impl IntoResponse for SamError {
//     fn into_response(self) -> Response {
//         let status = match self {
//             SamError::LoginFailed | SamError::NotAuthorized => StatusCode::UNAUTHORIZED,
//             SamError::Database { .. }
//             | SamError::Json(_)
//             | SamError::RegistrationFailed
//             | SamError::InvalidToken
//             | SamError::ExpiredToken(_) => StatusCode::BAD_REQUEST,
//             _ => StatusCode::INTERNAL_SERVER_ERROR,
//         };

//         (status, self.to_string()).into_response()
//     }
// }

pub async fn error_middleware(request: Request, next: Next) -> Response {
    let response = next.run(request).await;
    let status_code = response.status().as_u16();
    // Check if the response is an error
    if response.status().is_client_error() || response.status().is_server_error() {
        // Extract the body from the response
        let (_, body) = response.into_parts();

        if let Ok(body_bytes) = to_bytes(body, usize::MAX).await {
            // Try to parse the body as JSON
            match serde_json::from_slice::<UserResponse>(&body_bytes) {
                Ok(user_response) => user_response.into_error_response(),
                Err(_) => {
                    // If the body is not JSON, treat it as a plain string
                    let message = String::from_utf8(body_bytes.to_vec())
                        .unwrap_or_else(|_| "Unknown error".to_string());

                    UserResponse::new(message.as_str(), status_code).into_error_response()
                }
            }
        } else {
            //UserResponse::new("From here", 200).into_error_response()
            UserResponse::into_gen_error_response()
        }
    } else {
        response
    }
}

pub async fn handle_error(error: Box<dyn Any + Send>, pool: &PgPool) -> Response {
    Response::new("hello samoo...".into())
}
