// Recommended RESTful Routing for categories
// GET	/categories	List all categories
// GET	/categories/:id	Get a specific category
// POST	/categories	Create a new category
// PUT	/categories/:id	Update a category
// DELETE	/categories/:id	Delete a category

use std::collections::HashMap;

use super::category_db::*;
use crate::error::Result;
use crate::{response::*, AppState};
use axum::{
    extract::{rejection::JsonRejection, Path, Query, State},
    response::{IntoResponse, Response},
    routing::{delete, get, post},
    Json, Router,
};
use polars::prelude::{col, lit, DataType, IntoLazy};
use sam_error::SamError;
use sam_util::rows_to_dataframe;
use serde_json::{json, to_value};
use shared::{Category, CategoryName, CategoryWithNames};

pub fn category_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/categories",
            get(list_categories_handler).post(add_category_with_names_handler),
        )
        .route(
            "/categories/{id}",
            delete(delete_category_handler).get(get_category_handler),
        )
}

// async fn list_categories_handler(State(state): State<AppState>) -> Result<Response> {
//     let categories: Vec<Category> = list_categories(&state.pool).await?;
//     let res = UserResponse::with_json(categories).into_response();
//     Ok(res)
// }

// async fn list_categories_handler(State(state): State<AppState>) -> Result<Response> {
//     let df = list_categories(&state.pool).await?;

//     // Convert to JSON using Polars' built-in functionality
//     let json_str = df
//         .to_json(JsonFormat::Json)
//         .map_err(|e| sam_error::SamError::Err(e.to_string()));
//     let json_value: serde_json::Value = serde_json::from_str(&json_str)
//         .map_err(|e| sam_error::SamError::Err(e.to_string()))?;

//     let res = UserResponse::with_json(json_value).into_response();
//     Ok(res)
// }

// async fn list_categories_handler(State(state): State<AppState>) -> Result<Response> {
//     let df = list_categories(&state.pool).await?;

//     // Convert to Vec<HashMap<String, Value>>
//     let mut records = Vec::new();
//     for i in 0..df.height() {
//         let mut record = HashMap::new();
//         for col in df.get_columns() {
//             let value = match col.dtype() {
//                 DataType::String => col.str().unwrap().get(i).map(|s| json!(s)),
//                 DataType::Int32 => col.i32().unwrap().get(i).map(|n| json!(n)),
//                 DataType::Int64 => col.i64().unwrap().get(i).map(|n| json!(n)),
//                 DataType::Float32 => col.f32().unwrap().get(i).map(|n| json!(n)),
//                 DataType::Float64 => col.f64().unwrap().get(i).map(|n| json!(n)),
//                 DataType::Boolean => col.bool().unwrap().get(i).map(|b| json!(b)),
//                 _ => None,
//             };
//             record.insert(col.name().to_string(), value.unwrap_or(json!(null)));
//         }
//         records.push(record);
//     }

//     let res = UserResponse::with_json(records).into_response();
//     Ok(res)
// }

async fn list_categories_handler(State(state): State<AppState>) -> Result<Response> {
    let mut rows = list_categories(&state.pool).await?;
    let df = rows_to_dataframe(rows).await?;
    let mut filtered = df
        .lazy()
        .filter(col("id").str().contains(lit("a".to_string()), false))
        .select([col("id")])
        .collect()
        .map_err(|e| sam_error::SamError::Err(e.to_string()))?;

    let records = sam_util::df_to_json(&mut filtered).map_err(|e| SamError::Err(e.to_string()))?;

    let res = UserResponse::with_json(records).into_response();
    Ok(res)
}

async fn get_category_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response> {
    let category: Category = get_category(&state.pool, id).await?;
    let res = UserResponse::with_json(category).into_response();
    Ok(res)
}

// async fn add_category_handler(
//     State(state): State<AppState>,
//     Json(category): Json<Category>,
// ) -> Result<Response> {
//     add_category(&state.pool, category).await?;
//     let res = UserResponse::with_success("Category Added Successfully").into_response();
//     Ok(res)
// }

async fn add_category_with_names_handler(
    State(state): State<AppState>,
    Json(category_with_names): Json<CategoryWithNames>,
) -> Result<Response> {
    add_category_with_names(
        &state.pool,
        category_with_names.category,
        category_with_names.names,
    )
    .await?;
    let res = UserResponse::with_success("Category and Names Added Successfully").into_response();
    Ok(res)
}

async fn delete_category_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Response> {
    delete_category(&state.pool, id).await?;
    let res = UserResponse::with_success("Category Deleted Successfully").into_response();
    Ok(res)
}

async fn list_categories_names_handler(State(state): State<AppState>) -> Result<Response> {
    let categories_names: Vec<CategoryName> = list_categories_names(&state.pool).await?;
    let res = UserResponse::with_json(categories_names).into_response();
    Ok(res)
}
