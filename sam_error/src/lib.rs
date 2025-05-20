use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SamError {
    #[error("{0}")]
    Database(#[from] sqlx::Error),
    #[error("{0}")]
    InvalidJson(#[from] JsonRejection),
    #[error("Login failed: email or password is wrong")]
    LoginFailed,
    #[error("Registration failed.")]
    RegistrationFailed,
    #[error("Not authorized.Please login")]
    NotAuthorized,
    #[error("Invalid token.")]
    InvalidToken,
    #[error("Expired token.")]
    ExpiredToken(String),
    #[error("Missing Enviroment Variable: {0}")]
    MissingEnviromentVariable(String),
    #[error("Email Send Failed.")]
    EmailSendFailed,
    #[error("Email Not Found")]
    EmailNotFound,
    #[error("Invalid email.")]
    InvalidEmail,
    #[error("Invalid Password: {0}")]
    InvalidPassword(String),
    #[error("Something went wrong")]
    Any,
    #[error("{0}")]
    AnyWithContext(String),
}

impl IntoResponse for SamError {
    fn into_response(self) -> Response {
        let status = match self {
            SamError::LoginFailed | SamError::NotAuthorized => StatusCode::UNAUTHORIZED,
            SamError::InvalidJson(_)
            | SamError::RegistrationFailed
            | SamError::InvalidToken
            | SamError::InvalidEmail
            | SamError::InvalidPassword(_)
            | SamError::ExpiredToken(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, self.to_string()).into_response()
    }
}

impl From<String> for SamError {
    fn from(value: String) -> Self {
        SamError::AnyWithContext(value)
    }
}

/// # Log error and return general error
/// This macro return SamError::Any after logging the origin error   
/// We send SamError::Any to a client for avoiding misuse
#[macro_export]
macro_rules! any_with_log {
    ($err:expr) => {
        {
            tracing::error!("\nModule: {}\nFile: {}\nLine: {}\n🚨Error Message: \n-------------\n{}\n-------------", module_path!(),file!(),line!(),$err);
            $crate::SamError::Any
        }
    };
}
