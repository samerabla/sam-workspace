use super::language_db::*;
use crate::error::Result;
use crate::{response::*, AppState};
use axum::{
    extract::{rejection::JsonRejection, Path, Query, State},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use sam_error::SamError;
use serde_json::to_value;
use shared::Language;

pub fn language_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/languages",
            get(list_languages_handler)
                .post(add_language_handler)
                .put(update_language_handler),
        )
        .route(
            "/languages/{id}",
            delete(delete_language_handler).get(get_language_handler),
        )
}

async fn list_languages_handler(State(state): State<AppState>) -> Result<Response> {
    let languages: Vec<Language> = list_languages(&state.pool).await?;
    let res = UserResponse::with_json(languages).into_response();
    Ok(res)
}

async fn get_language_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response> {
    let language: Language = get_language(&state.pool, id).await?;
    let res = UserResponse::with_json(language).into_response();
    Ok(res)
}

async fn add_language_handler(
    State(state): State<AppState>,
    Json(language): Json<Language>,
) -> Result<Response> {
    add_language(&state.pool, language).await?;
    let res = UserResponse::with_success("Language Added Successfully").into_response();
    Ok(res)
}

async fn update_language_handler(
    State(state): State<AppState>,
    Json(language): Json<Language>,
) -> Result<Response> {
    update_language(&state.pool, language).await?;
    let res = UserResponse::with_success("Language Added Successfully").into_response();
    Ok(res)
}

async fn delete_language_handler(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Response> {
    delete_language(&state.pool, id).await?;
    let res = UserResponse::with_success("Language Deleted Successfully").into_response();
    Ok(res)
}
