// use super::Category;
use crate::error::Result;
use polars::prelude::{lit, IntoLazy};
use polars::{frame::DataFrame, prelude::col};
use sam_proc_macros::catch_error;
use sam_util::rows_to_dataframe;
use shared::{Category, CategoryName};
use sqlx::{query, query_as, PgPool};

#[catch_error]
pub async fn add_category_with_names(
    pool: &PgPool,
    category: Category,
    names: Vec<CategoryName>,
) -> Result<()> {
    let mut tx: sqlx::Transaction<'static, sqlx::Postgres> = pool.begin().await?;

    // Insert category
    add_category(&mut tx, category).await?;

    // Insert all names
    for name in names {
        add_category_name(&mut tx, name).await?;
    }

    tx.commit().await?;
    Ok(())
}

#[catch_error]
async fn add_category<'a>(
    tx: &mut sqlx::Transaction<'a, sqlx::Postgres>,
    category: Category,
) -> Result<()> {
    query!(
        r#"
        INSERT INTO categories (id,parent_id)
        VALUES ($1, $2) 
        "#,
        category.id,
        category.parent_id
    )
    .execute(&mut **tx)
    .await?;
    Ok(())
}

#[catch_error]
async fn add_category_name<'a>(
    tx: &mut sqlx::Transaction<'a, sqlx::Postgres>,
    category_name: CategoryName,
) -> Result<()> {
    query!(
        r#"
        INSERT INTO categories_names (name,language_id,category_id,slug)
        VALUES ($1, $2, $3, $4) 
        "#,
        category_name.name,
        category_name.language_id,
        category_name.category_id,
        category_name.slug
    )
    .execute(&mut **tx)
    .await?;
    Ok(())
}

#[catch_error]
pub async fn list_categories(pool: &PgPool) -> Result<Vec<sqlx::postgres::PgRow>> {
    let rows: Vec<sqlx::postgres::PgRow> =
        query(r#"SELECT * FROM categories"#).fetch_all(pool).await?;
    Ok(rows)
}

#[catch_error]
pub async fn list_categories1(pool: &PgPool) -> Result<Vec<Category>> {
    let categories: Vec<Category> = query_as!(
        Category,
        r#"
        SELECT * FROM categories 
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(categories)
}

#[catch_error]
pub async fn list_categories11(pool: &PgPool) -> Result<DataFrame> {
    let rows: Vec<sqlx::postgres::PgRow> =
        query(r#"SELECT * FROM categories"#).fetch_all(pool).await?;
    let df = rows_to_dataframe(rows).await?;

    // Now you can use Polars operations
    let filtered = df
        .lazy()
        .filter(col("id").str().contains(lit("a".to_string()), false))
        .select([col("id")])
        .collect()
        .map_err(|e| sam_error::SamError::Err(e.to_string()));
    filtered
}

#[catch_error]
pub async fn get_category(pool: &PgPool, category_id: String) -> Result<Category> {
    let category: Category = query_as!(
        Category,
        r#"
        SELECT * FROM categories 
        WHERE id = $1
        "#,
        category_id,
    )
    .fetch_one(pool)
    .await?;
    Ok(category)
}

// #[catch_error]
// pub async fn add_category(pool: &PgPool, category: Category) -> Result<()> {
//     query!(
//         r#"
//         INSERT INTO categories (id,parent_id)
//         VALUES ($1, $2)
//         "#,
//         category.id,
//         category.parent_id
//     )
//     .execute(pool)
//     .await?;
//     Ok(())
// }

#[catch_error]
pub async fn delete_category(pool: &PgPool, category_id: String) -> Result<()> {
    query!(
        r#"
        DELETE FROM categories 
        WHERE id = $1
        "#,
        category_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}

// #[catch_error]
// pub async fn add_category_name(pool: &PgPool, category_name: CategoryName) -> Result<()> {
//     query!(
//         r#"
//         INSERT INTO categories_names (name,language_id,category_id,slug)
//         VALUES ($1, $2, $3, $4)
//         "#,
//         category_name.name,
//         category_name.language_id,
//         category_name.category_id,
//         category_name.slug
//     )
//     .execute(pool)
//     .await?;
//     Ok(())
// }

#[catch_error]
pub async fn list_categories_names(pool: &PgPool) -> Result<Vec<CategoryName>> {
    let categories: Vec<CategoryName> = query_as!(
        CategoryName,
        r#"
        SELECT * FROM categories_names 
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(categories)
}
