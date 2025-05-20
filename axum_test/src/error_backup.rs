use sqlx::{Error as SqlxError, PgPool};
use std::{any::Any, fmt};
use tracing::error;

use axum::{
    body::{to_bytes, Body},
    extract::{rejection::JsonRejection, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};

use thiserror::Error;

use crate::{user::UserResponse, AppState};
pub type Result<T, E = SamError> = std::result::Result<T, E>;
type BoxError = Box<dyn Any + Send + 'static>;

// #[derive(Error, Debug)]
// pub enum SamError {
//     #[error("Hashing Error")]
//     Argon2,
//     #[error("Validation error: {0}")]
//     Validation(String),
//     #[error("Database error: {0}")]
//     Database(#[from] SqlxError),
//     #[error("Internal server error")]
//     Internal,
// }

// impl SamError {
//     pub fn status_code(&self) -> StatusCode {
//         match self {
//             SamError::Validation(_) => StatusCode::BAD_REQUEST,
//             SamError::Database(err) => {
//                 if let SqlxError::Database(db_err) = err {
//                     if db_err.constraint() == Some("users_email_key") {
//                         StatusCode::CONFLICT
//                     } else {
//                         StatusCode::INTERNAL_SERVER_ERROR
//                     }
//                 } else {
//                     StatusCode::INTERNAL_SERVER_ERROR
//                 }
//             }
//             _ => StatusCode::INTERNAL_SERVER_ERROR,
//         }
//     }
// }

#[derive(Error, Debug)]
pub enum SamError {
    #[error("SamError::Database >> {0}")]
    Database(#[from] SqlxError),
    #[error("SamError::Json >> {0}")]
    Json(#[from] JsonRejection),
    #[error("Internal server error")]
    Validation(String),
    #[error("Internal server error")]
    Internal(String),
}

// #[derive(Debug)]
// pub enum SamError {
//     Database(SqlxError),
//     Validation(String),
//     Internal(String),
// }

// impl fmt::Display for SamError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             SamError::Database(err) => write!(f, "Database error: {}", err),
//             SamError::Validation(msg) => write!(f, "Validation error: {}", msg),
//             SamError::Internal(msg) => write!(f, "Internal error: {}", msg),
//         }
//     }
// }

impl IntoResponse for SamError {
    fn into_response(self) -> Response {
        error!("🚨 {}: ", self);
        match self {
            SamError::Database(err) => match err {
                SqlxError::Database(db_err) if db_err.constraint() == Some("users_email_key") => {
                    UserResponse::new("Registration failed", 400).into_error_response()
                }
                _ => UserResponse::error(),
            },
            SamError::Json(err) => {
                UserResponse::new(err.body_text().as_str(), 400).into_error_response()
            }
            SamError::Validation(msg) => UserResponse::new(&msg, 400).into_error_response(),
            SamError::Internal(msg) => UserResponse::new(&msg, 500).into_error_response(),
        }
    }
}

pub async fn error_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;

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
                    UserResponse::new(message.as_str(), 500).into_error_response()
                }
            }
        } else {
            UserResponse::error()
        }
    } else {
        response
    }
}

// pub async fn error_middleware(request: Request, next: Next) -> Result<Response, SamError> {
//     // Run the request and get the response
//     let response = next.run(request).await;

//     // Check if the response is an error
//     if response.status().is_client_error() || response.status().is_server_error() {
//         // Extract the body from the response
//         let (parts, body) = response.into_parts();

//         // Convert the body to bytes
//         let body_bytes = match to_bytes(body).await {
//             Ok(bytes) => bytes,
//             Err(_) => {
//                 // If we can't read the body, return a generic error
//                 return Err(SamError::Internal(
//                     "Failed to read response body".to_string(),
//                 ));
//             }
//         };

//         // Try to parse the body as JSON
//         let error_message = match serde_json::from_slice::<Value>(&body_bytes) {
//             Ok(json) => {
//                 // If the body is JSON, extract the "error" field or use the entire JSON as a string
//                 json.get("error")
//                     .and_then(|v| v.as_str())
//                     .map(|s| s.to_string())
//                     .unwrap_or_else(|| json.to_string())
//             }
//             Err(_) => {
//                 // If the body is not JSON, treat it as a plain string
//                 String::from_utf8(body_bytes.to_vec())
//                     .unwrap_or_else(|_| "Unknown error".to_string())
//             }
//         };

//         // Return a validation error
//         Err(SamError::Validation(error_message))
//     } else {
//         // If it's not an error, return the response as-is
//         Ok(response)
//     }
// }

pub async fn handle_error(error: Box<dyn Any + Send>, pool: &PgPool) -> Response {
    Response::new("hello samoo...".into())
}

// pub async fn handle_error(error: Box<dyn Any + Send>, pool: &PgPool) -> Response {
//     // Log the error (optional)
//     println!("Error: {:?}", error);

//     // Determine the status code and message
//     let (status_code, message) = if let Some(sam_error) = error.downcast_ref::<SamError>() {
//         // If the error is a SamError, use its status code and message
//         (sam_error.status_code(), sam_error.to_string())
//     } else if let Some(sqlx_error) = error.downcast_ref::<SqlxError>() {
//         // Handle SQLx errors (e.g., duplicate email)
//         match sqlx_error {
//             SqlxError::Database(db_error) if db_error.constraint() == Some("users_email_key") => {
//                 // Duplicate email error
//                 (StatusCode::BAD_REQUEST, "Email already exists".to_string())
//             }
//             _ => {
//                 // Other database errors
//                 (
//                     StatusCode::INTERNAL_SERVER_ERROR,
//                     "Database error".to_string(),
//                 )
//             }
//         }
//     } else if let Some(panic_msg) = error.downcast_ref::<&str>() {
//         // If the error is a panic message (&str), use it directly
//         (StatusCode::INTERNAL_SERVER_ERROR, panic_msg.to_string())
//     } else if let Some(panic_msg) = error.downcast_ref::<String>() {
//         // If the error is a panic message (String), use it directly
//         (StatusCode::INTERNAL_SERVER_ERROR, panic_msg.to_string())
//     } else {
//         // Fallback for unknown error types
//         (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             "An unknown error occurred".to_string(),
//         )
//     };

//     // Return the error message as it is
//     let response = json!({
//         "status_code": status_code.as_u16(),
//         "message": message,
//     });

//     (status_code, Json(response)).into_response()
// }
