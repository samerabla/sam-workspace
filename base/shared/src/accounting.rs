// use serde::{Deserialize, Serialize};
// use time::OffsetDateTime;
// use uuid::Uuid;

// #[derive(Debug, Serialize, Deserialize)]
// #[cfg_attr(feature = "backend", derive(FromRow))]
// pub struct Account {
//     pub id: Uuid,
//     pub name: String,
//     pub code: String,
//     pub account_type: AccountType,
//     pub parent_id: Option<Uuid>,
//     pub is_active: bool,
//     pub created_at: OffsetDateTime,
//     pub updated_at: OffsetDateTime,
// }

// #[derive(Debug, Serialize, Deserialize, sqlx::Type)]
// #[sqlx(type_name = "account_type", rename_all = "lowercase")]
// pub enum AccountType {
//     Asset,
//     Liability,
//     Equity,
//     Revenue,
//     Expense,
// }
