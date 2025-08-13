use std::collections::HashMap;

use shared::{Field, FieldDataType};
use sqlx::{query, query_as, PgPool};

pub struct FieldService;

impl FieldService {
    /// Get field definition from database
    pub async fn get_field_definition(
        pool: &PgPool,
        field_id: &str,
    ) -> Result<Option<Field>, sqlx::Error> {
        let field = query_as!(
            Field,
            r#"
            SELECT 
                id,
                data_type as "data_type: FieldDataType",
                is_required,
                validation_rules,
                is_filterable,
                is_searchable,
                sort_order
            FROM fields 
            WHERE id = $1
            "#,
            field_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(field)
    }

    /// Insert field value automatically based on field type
    pub async fn insert_field_value(
        pool: &PgPool,
        listing_id: uuid::Uuid,
        field_id: &str,
        value: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Get field definition
        let field_def = Self::get_field_definition(pool, field_id)
            .await?
            .ok_or(format!("Field '{}' not found", field_id))?;

        // Validate if required
        if field_def.is_required && value.trim().is_empty() {
            return Err(format!("Field '{}' is required", field_id).into());
        }

        // Insert based on data type
        match field_def.data_type {
            FieldDataType::String => {
                query!(
                    r#"
                    INSERT INTO listing_field_values (listing_id, field_id, value_text)
                    VALUES ($1, $2, $3)
                    "#,
                    listing_id,
                    field_id,
                    value
                )
                .execute(pool)
                .await?;
            }

            FieldDataType::Integer => {
                let int_value: i32 = value
                    .parse()
                    .map_err(|_| format!("Invalid integer value: {}", value))?;

                query!(
                    r#"
                    INSERT INTO listing_field_values (listing_id, field_id, value_integer)
                    VALUES ($1, $2, $3)
                    "#,
                    listing_id,
                    field_id,
                    int_value
                )
                .execute(pool)
                .await?;
            }

            FieldDataType::Decimal => {
                let decimal_value: f64 = value
                    .parse()
                    .map_err(|_| format!("Invalid decimal value: {}", value))?;

                query!(
                    r#"
                    INSERT INTO listing_field_values (listing_id, field_id, value_decimal)
                    VALUES ($1, $2, $3)
                    "#,
                    listing_id,
                    field_id,
                    decimal_value
                )
                .execute(pool)
                .await?;
            }

            FieldDataType::Boolean => {
                let bool_value: bool = value
                    .parse()
                    .map_err(|_| format!("Invalid boolean value: {}", value))?;

                query!(
                    r#"
                    INSERT INTO listing_field_values (listing_id, field_id, value_boolean)
                    VALUES ($1, $2, $3)
                    "#,
                    listing_id,
                    field_id,
                    bool_value
                )
                .execute(pool)
                .await?;
            }

            FieldDataType::Date => {
                let date_value =
                    time::Date::parse(value, &time::format_description::well_known::Iso8601::DATE)
                        .map_err(|_| {
                            format!("Invalid date value: {} (expected YYYY-MM-DD)", value)
                        })?;

                query!(
                    r#"
                    INSERT INTO listing_field_values (listing_id, field_id, value_date)
                    VALUES ($1, $2, $3)
                    "#,
                    listing_id,
                    field_id,
                    date_value
                )
                .execute(pool)
                .await?;
            }

            FieldDataType::Select
            | FieldDataType::Multiselect
            | FieldDataType::Location
            | FieldDataType::File => {
                let json_value: serde_json::Value = serde_json::from_str(value)
                    .map_err(|_| format!("Invalid JSON value: {}", value))?;

                query!(
                    r#"
                    INSERT INTO listing_field_values (listing_id, field_id, value_json)
                    VALUES ($1, $2, $3)
                    "#,
                    listing_id,
                    field_id,
                    json_value
                )
                .execute(pool)
                .await?;
            }
        }

        Ok(())
    }

    /// Insert multiple field values at once
    pub async fn insert_multiple_field_values(
        pool: &PgPool,
        listing_id: uuid::Uuid,
        field_values: HashMap<String, String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (field_id, value) in field_values {
            Self::insert_field_value(pool, listing_id, &field_id, &value).await?;
        }
        Ok(())
    }

    /// Get all field values for a listing
    pub async fn get_listing_field_values(
        pool: &PgPool,
        listing_id: uuid::Uuid,
    ) -> Result<HashMap<String, serde_json::Value>, sqlx::Error> {
        let rows = query!(
            r#"
            SELECT 
                field_id,
                COALESCE(
                    to_jsonb(value_text),
                    to_jsonb(value_integer),
                    to_jsonb(value_decimal),
                    to_jsonb(value_boolean),
                    to_jsonb(value_date),
                    to_jsonb(value_datetime),
                    value_json
                ) as value
            FROM listing_field_values
            WHERE listing_id = $1
            "#,
            listing_id
        )
        .fetch_all(pool)
        .await?;

        let mut result = HashMap::new();
        for row in rows {
            if let Some(value) = row.value {
                result.insert(row.field_id, value);
            }
        }

        Ok(result)
    }
}
