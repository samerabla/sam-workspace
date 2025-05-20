use sqlx::Error as SqlxError;
use std::any::Any;

use axum::{
    body::Body,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde_json::{json, Value};
use sqlx::PgPool;

use crate::AppState;
use axum::{body::to_bytes, extract::Request, middleware::Next};
use std::sync::Arc;

pub type Result<T, E = SamError> = std::result::Result<T, E>;
type BoxError = Box<dyn Any + Send + 'static>;

// #[derive(thiserror::Error, Debug)]
// pub enum Error {
//     #[error("Hashing Error")]
//     Argon2,
//     #[error("Internal Server Error")]
//     InternalServerError,
//     #[error("{0:?}")]
//     Any(String),
// }

// #[derive(Debug)]
// pub struct ErrorDetails {
//     module: String,
//     file: String,
//     line: u32,
//     message: String,
// }

// impl ErrorDetails {
//     pub fn new(module: &str, file: &str, line: u32, message: String) -> Self {
//         Self {
//             module: module.into(),
//             file: file.into(),
//             line,
//             message: message,
//         }
//     }
// }

//-------------------

#[derive(thiserror::Error, Debug)]
pub enum SamError {
    #[error("Hashing Error")]
    Argon2,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Internal server error")]
    Internal,
}

impl SamError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            SamError::Validation(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for SamError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let message = self.to_string();

        let response = json!({
            "status_code": status_code.as_u16(),
            "message": message,
        });

        (status_code, Json(response)).into_response()
    }
}

pub async fn handle_error(error: Box<dyn Any + Send>, pool: &PgPool) -> Response {
    // Log the error (optional)
    println!("Error: {:?}", error);

    // Determine the status code and message
    let (status_code, message) = if let Some(sam_error) = error.downcast_ref::<SamError>() {
        // If the error is a SamError, use its status code and message
        log_error_chain(&sam_error);
        (sam_error.status_code(), sam_error.to_string())
    } else if let Some(sqlx_error) = error.downcast_ref::<SqlxError>() {
        // Handle SQLx errors (e.g., duplicate email)
        match sqlx_error {
            SqlxError::Database(db_error) if db_error.constraint() == Some("users_email_key") => {
                // Duplicate email error
                (StatusCode::BAD_REQUEST, "Email already exists".to_string())
            }
            _ => {
                // Other database errors
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }
        }
    } else if let Some(panic_msg) = error.downcast_ref::<&str>() {
        // If the error is a panic message (&str), use it directly
        (StatusCode::INTERNAL_SERVER_ERROR, panic_msg.to_string())
    } else if let Some(panic_msg) = error.downcast_ref::<String>() {
        // If the error is a panic message (String), use it directly
        (StatusCode::INTERNAL_SERVER_ERROR, panic_msg.to_string())
    } else {
        // Fallback for unknown error types
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "An unknown error occurred".to_string(),
        )
    };

    // Return the error message as it is
    let response = json!({
        "status_code": status_code.as_u16(),
        "message": message,
    });

    (status_code, Json(response)).into_response()
}

// pub async fn handle_error(error: Box<dyn Any + Send>, pool: &PgPool) -> Response {
//     // Log the error (optional)
//     println!("Error: {:?}", error);

//     // Determine the status code
//     let status_code = if let Some(app_error) = error.downcast_ref::<SamError>() {
//         app_error.status_code()
//     } else {
//         StatusCode::INTERNAL_SERVER_ERROR
//     };

//     // Extract the error message directly
//     let message = if let Some(sam_error) = error.downcast_ref::<SamError>() {
//         // If the error is a SamError, use its Display implementation
//         sam_error.to_string()
//     } else if let Some(panic_msg) = error.downcast_ref::<&str>() {
//         // If the error is a panic message (&str), use it directly
//         panic_msg.to_string()
//     } else if let Some(panic_msg) = error.downcast_ref::<String>() {
//         // If the error is a panic message (String), use it directly
//         panic_msg.to_string()
//     } else {
//         // Fallback for unknown error types
//         "An unknown error occurred".to_string()
//     };

//     // Return the error message as it is
//     let response = json!({
//         "status_code": status_code.as_u16(),
//         "message": message,
//     });

//     (status_code, Json(response)).into_response()
// }

// pub async fn handle_error(error: BoxError, pool: &PgPool) -> Response {
//     // Log the error
//     //log_err(pool, module_path!(), file!(), line!(), &error).await;
//     // Determine the status code and message
//     println!("{:#?}", error);
//     let status_code = if let Some(app_error) = error.downcast_ref::<SamError>() {
//         app_error.status_code()
//     } else {
//         StatusCode::INTERNAL_SERVER_ERROR
//     };

//     let message = format!("{:?}", error);

//     // Return a structured error response
//     let response = json!({
//         "status_code": status_code.as_u16(),
//         "message": message,
//     });

//     (status_code, Json(response)).into_response()
// }

async fn log_err(pool: &PgPool, module: &str, file: &str, line: u32, error: &BoxError) {
    let query = sqlx::query!(
        "INSERT INTO errors (module, file, line, message) VALUES ($1, $2, $3, $4)",
        module,
        file,
        line as i32,
        format!("{:?}", error)
    )
    .execute(pool)
    .await;

    if let Err(e) = query {
        eprintln!("Failed to log error: {}", e);
    }
}

//-----

pub async fn error_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    // Run the request and get the response
    let response = next.run(request).await;
    // Check if the response is an error
    if response.status().is_client_error() || response.status().is_server_error() {
        // Extract the body from the response
        let (_, body) = response.into_parts();

        if let Ok(body_bytes) = to_bytes(body, usize::MAX).await {
            // Try to parse the body as JSON
            let error_message = match serde_json::from_slice::<Value>(&body_bytes) {
                Ok(json) => {
                    // If the body is JSON, extract the "error" field or use the entire JSON as a string
                    json.get("error")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| json.to_string())
                }
                Err(_) => {
                    // If the body is not JSON, treat it as a plain string
                    String::from_utf8(body_bytes.to_vec())
                        .unwrap_or_else(|_| "Unknown error".to_string())
                }
            };

            let sam_error = SamError::Validation(error_message);
            return handle_error(Box::new(sam_error), &state.pool).await;
        }
        // //Response::from_parts(parts, Body::from("body samoora...."))
        (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong").into_response()
    } else {
        response
    }
}

use std::error::Error as StdError;

fn log_error_chain(error: &dyn StdError) {
    println!("Error chain:");
    let mut current_error: &dyn StdError = error;
    loop {
        println!("- {}", current_error);
        if let Some(source) = current_error.source() {
            current_error = source;
        } else {
            break;
        }
    }
}

// pub async fn error_middleware(
//     State(state): State<AppState>,
//     request: Request,
//     next: Next,
// ) -> Response {
//     // Run the request and get the response
//     let response = next.run(request).await;

//     // Check if the response is an error
//     if response.status().is_client_error() || response.status().is_server_error() {
//         // Create a SamError from the error message
//         let sam_error = SamError::Validation("error_message".to_string());

//         // Handle the error
//         handle_error(Box::new(sam_error), &state.pool).await;
//         // Reconstruct the response
//         response
//     } else {
//         response
//     }
// }

// fn app(pool: PgPool) -> Router {
//     Router::new()
//         .route("/users", post(add_user_handler))
//         .layer(CatchPanicLayer::custom(move |error| {
//             let pool = pool.clone();
//             async move {
//                 handle_error(error, &pool).await
//             }
//         }))
// }
