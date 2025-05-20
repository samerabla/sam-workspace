use std::sync::Arc;

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
use sam_error::SamError;

use crate::{error::Result, user::user_db::fetch_user_by_id, AppState};

use super::jwt::validate_jwt;

pub async fn auth_middleware(
    cookies: CookieJar,
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response> {
    // Extract the token if exist
    let token = cookies
        .get("token")
        .map(|c| c.value())
        .ok_or(SamError::NotAuthorized)?;

    // Validate the token and get the claims
    let claims = validate_jwt(token)?;

    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|err| sam_error::any_with_log!(err.to_string()))?;
    let user = fetch_user_by_id(&state.pool, user_id).await?;

    // Add claims contains user info to req
    req.extensions_mut().insert(Arc::new(user));

    Ok(next.run(req).await)
}

// pub async fn auth_middleware(cookies: CookieJar, req: Request, next: Next) -> Response {
//     // Extract the token if exist
//     let token = cookies
//         .get("token")
//         .map(|c| c.value())
//         .ok_or(SamError::NotAuthorized);
//     let res = SamError::NotAuthorized.into_response();
//     if token.is_err() {
//         return res;
//     };
//     // Validate the token
//     let claims = validate_jwt(token.unwrap());
//     if claims.is_err() {
//         return res;
//     }
//     // ToDO: get the user and add it to the req

//     next.run(req).await
// }
