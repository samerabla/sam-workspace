use crate::error::Result;
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand::rngs::OsRng;
use sam_error::SamError;

//pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
pub fn hash_password(password: impl Into<String>) -> Result<String> {
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Configure Argon2 with default parameters
    let argon2 = Argon2::default();

    // Hash the password
    let password_hash = argon2
        .hash_password(password.into().as_bytes(), &salt)
        .map_err(|err| sam_error::any_with_log!(err.to_string()))?;

    // Return the hashed password as a string
    Ok(password_hash.to_string())
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    // Parse the stored hash into a `PasswordHash` struct
    let parsed_hash = PasswordHash::new(hash).map_err(|_| SamError::LoginFailed)?;

    // Verify the password against the hash
    let verify_result = Argon2::default().verify_password(password.as_bytes(), &parsed_hash);
    match verify_result {
        Ok(_) => Ok(true),
        Err(_) => Err(SamError::LoginFailed),
    }
}
