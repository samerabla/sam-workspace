use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "backend", derive(sqlx::Type))]
#[cfg_attr(
    feature = "backend",
    sqlx(type_name = "field_data_type", rename_all = "lowercase")
)]
pub enum FieldDataType {
    String,
    Integer,
    Decimal,
    Select,
    Multiselect,
    Boolean,
    Date,
    File,
    Location,
}

#[derive(Debug, Serialize, Deserialize)]
// #[cfg_attr(feature = "backend", derive(FromRow))]
pub struct Field {
    pub id: String,
    pub data_type: FieldDataType,
    pub validation_rules: Option<serde_json::Value>,
    pub is_required: bool,
    pub is_searchable: bool,
    pub is_filterable: bool,
    pub sort_order: i32,
}

#[derive(Debug, Serialize, Deserialize)]
// #[cfg_attr(feature = "backend", derive(FromRow))]
pub struct FieldName {
    pub id: String,
    pub name: String,
    pub placeholder: Option<String>,
    pub language_id: String,
    pub field_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
// #[cfg_attr(feature = "backend", derive(FromRow))]
pub struct FieldOption {
    pub id: String,
    pub field_id: String,
    pub option_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
// #[cfg_attr(feature = "backend", derive(FromRow))]
pub struct FieldOptionName {
    pub id: String,
    pub option_id: String,
    pub language_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListingFieldValue {
    pub listing_id: uuid::Uuid,
    pub field_id: String,
    pub value: String, // Raw value as string
}

/// This struct is used to link a category to a field definition.
/// This is a many-to-many relationship, where a category can have multiple fields and a field can belong to multiple categories.
/// We use a separate struct from `CategoryFieldDef` because we can have fields that belong to many categories at the same time.
#[derive(Debug, Serialize, Deserialize)]
// #[cfg_attr(feature = "backend", derive(FromRow))]
pub struct CategoryField {
    pub id: String,
    pub category_id: String,
    pub field_id: String,
}

#[cfg(feature = "backend")]
pub mod backend {}
