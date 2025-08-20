use crate::error::Result;
use sam_proc_macros::catch_error;
use shared::Language;
use sqlx::{query, query_as, PgPool};

#[catch_error]
pub async fn list_languages(pool: &PgPool) -> Result<Vec<Language>> {
    let languages: Vec<Language> = query_as!(
        Language,
        r#"
        SELECT * FROM languages 
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(languages)
}

#[catch_error]
pub async fn get_language(pool: &PgPool, language_id: String) -> Result<Language> {
    let language: Language = query_as!(
        Language,
        r#"
        SELECT * FROM languages 
        WHERE id = $1
        "#,
        language_id,
    )
    .fetch_one(pool)
    .await?;
    Ok(language)
}

#[catch_error]
pub async fn add_language(pool: &PgPool, language: Language) -> Result<()> {
    query!(
        r#"
        INSERT INTO languages (id,name,flag,active)
        VALUES ($1, $2, $3, $4) 
        "#,
        language.id,
        language.name,
        language.flag,
        language.active
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[catch_error]
pub async fn update_language(pool: &PgPool, language: Language) -> Result<()> {
    query!(
        r#"
        UPDATE languages
        SET
            name = $2,
            flag = $3,
            active = $4
        WHERE id = $1
        "#,
        language.id,
        language.name,
        language.flag,
        language.active
    )
    .execute(pool)
    .await?;
    Ok(())
}

#[catch_error]
pub async fn delete_language(pool: &PgPool, language_id: String) -> Result<()> {
    query!(
        r#"
        DELETE FROM languages 
        WHERE id = $1
        "#,
        language_id,
    )
    .execute(pool)
    .await?;
    Ok(())
}
