use super::jwt::create_jwt;
use super::user_routes::{ResetPasswordPayload, TokenInfo};
use super::{jwt::validate_jwt, AddUser, HashUser, User, UserInfo};
use crate::error::*;
use sam_error::any_with_log;
use sam_error::SamError;
use sam_proc_macros::catch_error;
use sqlx::{query, query_as, PgPool, Postgres};
use std::error::Error;
use std::sync::Arc;
use uuid::Uuid;

#[catch_error]
pub async fn add_pending_user(
    pool: &PgPool,
    user: AddUser,
    verification_token: String,
) -> Result<()> {
    let id = Uuid::new_v4();
    query!(
        r#"
        INSERT INTO pending_users (id,email, password, verification_token) 
        VALUES ($1, $2, $3,$4)
        "#,
        id,
        user.email,
        user.password,
        verification_token
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[catch_error]
pub async fn move_pending_user(pool: &PgPool, user: HashUser) -> Result<()> {
    // Start a transaction
    let mut transaction = pool.begin().await?;
    let id = user.id;

    // Add the user to the `users` table
    let add_result = query!(
        r#"
        INSERT INTO users (id, email, password) 
        VALUES ($1, $2, $3)
        "#,
        user.id,
        user.email,
        user.password,
    )
    .execute(&mut *transaction) // Use `&mut transaction` here
    .await;

    // Handle the result of the insert operation
    if let Err(err) = add_result {
        transaction.rollback().await?; // Rollback on error
        return Err(err.into());
    }

    // Delete the pending user from the `pending_users` table
    let delete_result = query!(
        r#"
        DELETE FROM pending_users
        WHERE id = $1
        "#,
        id
    )
    .execute(&mut *transaction) // Use `&mut transaction` here
    .await;

    // Handle the result of the delete operation
    if let Err(err) = delete_result {
        transaction.rollback().await?; // Rollback on error
        return Err(err.into());
    }

    // Commit the transaction if everything succeeds
    transaction.commit().await?;
    Ok(())
}

#[catch_error]
pub async fn fetch_user_by_email<S>(pool: &PgPool, email: S) -> Result<UserInfo>
where
    S: Into<String>,
{
    let user: UserInfo = query_as!(
        UserInfo,
        r#"
        SELECT id, email, created_at
        FROM users
        WHERE email = $1
        "#,
        email.into()
    )
    .fetch_one(pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => SamError::EmailNotFound,
        _ => any_with_log!(err.to_string()),
    })?;
    Ok(user)
}

pub async fn user_exists(pool: &PgPool, email: impl Into<String>) -> Result<bool> {
    match fetch_user_by_email(pool, email.into()).await {
        Ok(_) => Ok(true),
        Err(err) => match err {
            SamError::EmailNotFound => Ok(false),
            _ => Err(err),
        },
    }
}

// pub async fn user_exists(pool: &PgPool, email: impl Into<String>) -> bool {
//     match fetch_user_by_email(pool, email.into()).await {
//         Ok(_) => true,
//         Err(_) => false,
//     }
// }

#[catch_error]
pub async fn fetch_user_by_id(pool: &PgPool, id: Uuid) -> Result<UserInfo> {
    let user: UserInfo = query_as!(
        UserInfo,
        r#"
        SELECT id, email, created_at
        FROM users
        WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => SamError::LoginFailed,
        _ => any_with_log!(err.to_string()),
    })?;
    Ok(user)
}

// We need this for fetch the hashed password
#[catch_error]
pub async fn fetch_hash_user_by_email<S>(pool: &PgPool, email: S) -> Result<HashUser>
where
    S: Into<String>,
{
    let user: HashUser = query_as!(
        HashUser,
        r#"
        SELECT id,email, password
        FROM users
        WHERE email = $1
        "#,
        email.into()
    )
    .fetch_one(pool)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => SamError::LoginFailed,
        _ => sam_error::any_with_log!(err.to_string()),
    })?;
    Ok(user)
}

#[catch_error]
pub async fn fetch_pending_user<S>(pool: &PgPool, email: S) -> Result<HashUser>
where
    S: Into<String>,
{
    let user: HashUser = query_as!(
        HashUser,
        r#"
        SELECT id, password, email
        FROM pending_users
        WHERE email = $1
        "#,
        email.into()
    )
    .fetch_one(pool)
    .await?;
    Ok(user)
}

#[catch_error]
pub async fn reset_email_verification_token<S>(
    pool: &PgPool,
    verification_token: S,
    email: S,
) -> Result<()>
where
    S: Into<String>,
{
    query!(
        r#"
        UPDATE pending_users
        SET verification_token = $1
        WHERE email = $2
        "#,
        verification_token.into(),
        email.into()
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[catch_error]
pub async fn add_reset_password_token(pool: &PgPool, email: String, token: String) -> Result<()> {
    query!(
        r#"
        insert into password_reset_tokens (email,token) 
        values ($1,$2); 
    "#,
        email,
        token
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[catch_error]
pub async fn fetch_reset_password_token(pool: &PgPool, token: String) -> Result<TokenInfo> {
    let token_info: TokenInfo = query_as!(
        TokenInfo,
        r#"
        select token, email from password_reset_tokens
        where token = $1
        "#,
        token
    )
    .fetch_one(pool)
    .await?;
    Ok(token_info)
}

#[catch_error]
pub async fn reset_password(pool: &PgPool, hash: String, email: String) -> Result<()> {
    query!(
        r#"
        UPDATE users
        SET password = $1
        WHERE email = $2
        "#,
        hash,
        email
    )
    .execute(pool)
    .await?;

    Ok(())
}

#[catch_error]
pub async fn delete_token(pool: &PgPool, token: String) -> Result<()> {
    query!(
        r#"
        delete from password_reset_tokens
        WHERE token = $1
        "#,
        token
    )
    .execute(pool)
    .await?;

    Ok(())
}

// pub async fn cleanup_unverified_users(pool: Arc<PgPool>) {
//     loop {
//         // Delete unverified users older than 24 hours
//         let _ = query!(
//             r#"
//             DELETE FROM users
//             WHERE verified = false AND created_at < NOW() - INTERVAL '30 seconds'
//             "#
//         )
//         .execute(&*pool)
//         .await;

//         // Sleep for a while before running the cleanup again
//         //tokio::time::sleep(std::time::Duration::from_secs(60 * 60)).await; // Run every hour
//     }
// }
