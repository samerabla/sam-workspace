use sam_error::SamError;
use sam_util::gmail::{send, GmailConfig};

use crate::error::Result;

pub fn generate_verify_email_body(verification_token: String) -> Result<String> {
    let host = dotenvy::var("HOST")
        .map_err(|_| SamError::MissingEnviromentVariable("HOST".to_string()))?;
    let body = format!(
        r#"
        <a href="{}/users/verify-email?token={}">verify your email</a>
        "#,
        host, verification_token
    );
    Ok(body)
}

pub fn generate_forgot_password_body(verification_token: String) -> Result<String> {
    let host = dotenvy::var("HOST")
        .map_err(|_| SamError::MissingEnviromentVariable("HOST".to_string()))?;
    let body = format!(
        r#"
        <a href="{}/users/reset-password?token={}">Request a new password</a>
        "#,
        host, verification_token
    );
    Ok(body)
}

pub async fn send_verification_email(
    body: String,
    subject: impl Into<String>,
    user_email: &str,
) -> Result<()> {
    let email_sender = dotenvy::var("EMAIL_SENDER")
        .map_err(|_| SamError::MissingEnviromentVariable("EMAIL_SENDER".to_string()))?;
    let email_sender_password = dotenvy::var("EMAIL_SENDER_PASSWORD")
        .map_err(|_| SamError::MissingEnviromentVariable("EMAIL_SENDER_PASSWORD".to_string()))?;

    let config = GmailConfig {
        username: email_sender.clone(),
        password: email_sender_password,
        from: email_sender,
        to: user_email.to_string(),
        subject: subject.into(),
        body,
    };
    send(config).await.map_err(|_| SamError::EmailSendFailed)?;
    Ok(())
}
