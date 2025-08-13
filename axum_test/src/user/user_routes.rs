use sam_error::SamError;
use sam_util::validators::{validate_email, validate_password};

use super::{
    cookie::create_cookie,
    jwt::{create_jwt, validate_jwt},
    password::{hash_password, verify_password},
    user_db::{
        add_pending_user, add_reset_password_token, delete_token, fetch_hash_user_by_email,
        fetch_pending_user, fetch_reset_password_token, fetch_user_by_email, move_pending_user,
        reset_email_verification_token, reset_password, user_exists,
    },
    user_emails::{
        generate_forgot_password_body, generate_verify_email_body, send_verification_email,
    },
    Claims, LoginUser, LoginUser,
};
use crate::{error::Result, response::UserResponse, AppState};
use axum::{
    extract::{rejection::JsonRejection, Query, State},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::CookieJar;
use sam_proc_macros::catch_error;
use serde::Deserialize;
use sqlx::FromRow;

pub fn user_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/users/test", get(|| async { "Hello" }))
        .route("/users/add", post(add_user_handler))
        .route("/users/login", post(login_user_handler))
        .route("/users/logout", post(logout_user_handler))
        .route("/users/verify-email", get(verify_email_handler))
        .route(
            "/users/resend-verification",
            get(resend_verification_handler),
        )
        .route("/users/forgot-password", post(forgot_password_handler))
        .route("/users/reset-password", post(reset_password_handler))
        .with_state(state)
}

async fn add_user_handler(
    State(state): State<AppState>,
    user: Result<Json<LoginUser>, JsonRejection>,
) -> Result<Response> {
    // let user = user?.0.hash()?;
    let user = user?.0;
    validate_email(&user.email).map_err(|err| SamError::InvalidEmail)?;
    validate_password(&user.password, 8).map_err(|err| SamError::InvalidPassword(err))?;

    // Hash the password
    let user = user.hash()?;
    let email = user.email.clone();
    if user_exists(&state.pool, &email).await? {
        // todo: modify the message to be ambiguous
        return Err(sam_error::any_with_log!("User already found...".to_string()));
    }

    // Generate a verification token
    let verification_token = create_jwt(&email, 24 * 60 * 60)?; // valid for 24 hours

    // Add the user to the pending users table until he verifies its email.
    add_pending_user(&state.pool, user, verification_token.clone()).await?;

    // Send email for verifying
    let body = generate_verify_email_body(verification_token)?;
    send_verification_email(body, "Verify Your Email", &email).await?;

    let res = UserResponse::new(
        "User added successfully! The only step left is to check your email and verify it.",
        201,
    )
    .into_success_response();
    Ok(res)
}

async fn login_user_handler(
    State(s): State<AppState>,
    cookies: CookieJar,
    user: Result<Json<LoginUser>, JsonRejection>,
) -> Result<Response> {
    let LoginUser { email, password } = user?.0;
    let user = fetch_hash_user_by_email(&s.pool, email).await?;
    let hash = user.password;
    let user_id = user.id;

    verify_password(password.as_str(), hash.as_str())?;

    // Create jwt from id then store it in a cookie
    // Token expiration in seconds
    let seconds = 60 * 60;
    let duration = seconds as usize;
    let token = create_jwt(&user_id.to_string(), duration)?;
    let cookie = create_cookie("token".to_string(), token, seconds);
    let cookies = cookies.add(cookie);

    let res = UserResponse::new("Logedin Successfully", 200).into_success_response();
    Ok((cookies, res).into_response())
}

async fn logout_user_handler(cookies: CookieJar) -> Result<Response> {
    // Create an expired cookie to remove the JWT
    let expired_cookie = create_cookie("token".to_string(), "".to_string(), 0);
    let cookies = cookies.add(expired_cookie);
    let res = UserResponse::new("Logged out successfully", 200).into_success_response();

    Ok((cookies, res).into_response())
}

#[derive(Debug, Deserialize)]
pub struct VerifyEmailParams {
    token: String,
}

pub async fn verify_email_handler(
    Query(params): Query<VerifyEmailParams>,
    State(state): State<AppState>,
) -> Result<Response> {
    // Validate the verification token
    let claims = validate_jwt(&params.token)?;

    // Fetch user data from pending users table
    let user = fetch_pending_user(&state.pool, claims.sub).await?;

    // Add the user to the 'users' table and delete it from the 'pending_users' table
    move_pending_user(&state.pool, user).await?;

    let res = UserResponse::new("Email verified successfully", 200).into_success_response();
    Ok(res)
}

pub async fn resend_verification_handler(
    Query(params): Query<VerifyEmailParams>,
    State(state): State<AppState>,
) -> Result<Response> {
    // Extract the email from the expired token
    let email = match validate_jwt(&params.token) {
        Ok(token) => token.sub,
        Err(err) => match err {
            SamError::ExpiredToken(email) => email,
            _ => return Err(SamError::InvalidToken),
        },
    };

    // Create a new token
    let verification_token = create_jwt(&email, 24 * 60 * 60)?; // valid for 24 hours

    reset_email_verification_token(&state.pool, &verification_token, &email).await?;

    // Send email for verifying
    let body = generate_verify_email_body(verification_token)?;
    send_verification_email(body, "Verify Your Email", &email).await?;
    let res = UserResponse::new("Resent successfully! check your email and verify it.", 200)
        .into_success_response();
    Ok(res)
}

#[derive(Debug, Deserialize)]
pub struct ForgotPasswordPayload {
    pub email: String,
}

#[catch_error]
pub async fn forgot_password_handler(
    State(state): State<AppState>,
    Json(payload): Json<ForgotPasswordPayload>,
) -> Result<Response> {
    let user = match fetch_user_by_email(&state.pool, payload.email).await {
        Ok(user) => user,
        // We use an ambiguous message with ok status code for security purposes.
        Err(_) => return Ok(UserResponse::new("Check Your Email Box", 200).into_success_response()),
    };
    let email = user.email;
    let verification_token = create_jwt(&email, 60 * 60)?;
    let body = generate_forgot_password_body(verification_token.clone())?;
    send_verification_email(body, "Forgot Password", &email).await?;
    add_reset_password_token(&state.pool, email, verification_token).await?;
    Ok(UserResponse::new("Check Your Email Box", 200).into_success_response())
}

#[derive(Deserialize)]
pub struct ResetPasswordPayload {
    token: String,
    new_password: String,
}

#[derive(Debug, FromRow)]
pub struct TokenInfo {
    pub token: String,
    pub email: String,
}

#[catch_error]
pub async fn reset_password_handler(
    State(state): State<AppState>,
    payload: Result<Json<ResetPasswordPayload>, JsonRejection>,
) -> Result<Response> {
    let payload = payload?;
    let Json(ResetPasswordPayload {
        token,
        new_password,
    }) = payload;

    // Check if token exists
    let token_info = fetch_reset_password_token(&state.pool, token).await?;

    // Validate token
    let _claims: Claims = validate_jwt(&token_info.token)?;

    // Hash password
    let hash = hash_password(new_password)?;

    // Reset password
    reset_password(&state.pool, hash, token_info.email).await?;

    // Remove token from db
    delete_token(&state.pool, token_info.token).await?;

    Ok(UserResponse::new("Password reset successfully", 200).into_success_response())
}
