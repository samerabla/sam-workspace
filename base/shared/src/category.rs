use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[cfg_attr(feature = "backend", derive(sqlx::FromRow))]
pub struct Category {
    pub id: String,
    pub parent_id: Option<String>,
    // pub updated_at: time::OffsetDateTime,
}

impl Category {
    pub fn new(id: String, parent_id: Option<String>) -> Self {
        Self { id, parent_id }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CategoryName {
    pub id: Option<i32>,
    pub name: String,
    pub language_id: String,
    pub category_id: String,
    pub slug: String,
    // pub updated_at: time::OffsetDateTime,
}

impl CategoryName {
    pub fn new(name: String, language_id: String, category_id: String, slug: String) -> Self {
        Self {
            id: None,
            name,
            language_id,
            category_id,
            slug,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryWithNames {
    pub category: Category,
    pub names: Vec<CategoryName>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Language {
    pub id: String,
    pub name: String,
    pub flag: String,
    pub active: bool,
}
