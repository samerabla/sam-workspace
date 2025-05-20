use sam_error::SamError;

use crate::error::Result;
use jsonwebtoken::{
    decode, decode_header, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// Secret key for signing and verifying JWTs
pub const SECRET_KEY: &str = "your-secret-key";

// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (e.g., user ID)
    pub exp: usize,  // Expiration time
}

// Generate a JWT
pub fn create_jwt(sub: &str, duration: usize) -> Result<String> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| SamError::Any)?
        .as_secs() as usize
        + duration;

    let claims = Claims {
        sub: sub.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    )
    .map_err(|_| SamError::Any)
}

// pub fn validate_jwt(token: &str) -> Result<Claims> {
//     decode::<Claims>(
//         token,
//         &DecodingKey::from_secret(SECRET_KEY.as_bytes()),
//         &Validation::new(Algorithm::HS256),
//     )
//     .map(|data| data.claims)
//     .map_err(|_| SamError::NotAuthorized)
// }

pub fn validate_jwt(token: &str) -> Result<Claims> {
    // Decode the token header to check if it's a valid JWT
    let header = decode_header(token).map_err(|_| SamError::InvalidToken)?;

    // Validate algorithm (should match what is used for signing)
    if header.alg != Algorithm::HS256 {
        return Err(SamError::InvalidToken);
    }

    // Create decoding key
    let decoding_key = DecodingKey::from_secret(SECRET_KEY.as_bytes());

    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;

    // Decode token without expiration check to always retrieve `sub`
    let token_data =
        decode::<Claims>(token, &decoding_key, &validation).map_err(|_| SamError::Any)?;

    // Get current time (in seconds)
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| SamError::InvalidToken)?
        .as_secs() as usize;

    // Check if token is expired
    if token_data.claims.exp < now {
        return Err(SamError::ExpiredToken(token_data.claims.sub));
    }

    Ok(token_data.claims)
}
