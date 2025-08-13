pub mod category_db;
mod category_routes;
pub use category_routes::category_routes;

// use serde::{Deserialize, Serialize};
// use sqlx::FromRow;

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct Category {
//     pub id: String,
//     pub parent_id: Option<String>,
//     // pub updated_at: time::OffsetDateTime,
// }
