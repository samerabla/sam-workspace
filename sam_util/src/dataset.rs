use polars::io::json::JsonWriter;
use polars::prelude::JsonFormat;
use polars::{
    frame::DataFrame,
    prelude::{AnyValue, SerWriter},
    series::Series,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Column, Row};
use std::collections::HashMap;

pub async fn rows_to_dataframe(rows: Vec<sqlx::postgres::PgRow>) -> Result<DataFrame, String> {
    if rows.is_empty() {
        return Ok(DataFrame::empty());
    }

    let column_names: Vec<String> = rows[0]
        .columns()
        .iter()
        .map(|col| col.name().to_string())
        .collect();

    // Store all data as owned values to avoid lifetime issues
    let mut columns_data: HashMap<String, Vec<AnyValue<'static>>> = HashMap::new();

    // Initialize columns
    for col_name in &column_names {
        columns_data.insert(col_name.clone(), Vec::new());
    }

    // Process each row
    for row in &rows {
        for (i, _column) in row.columns().iter().enumerate() {
            let col_name = &column_names[i];
            let col_vec = columns_data.get_mut(col_name).unwrap();

            // Handle different PostgreSQL types
            // Try different types in order, starting with the most common ones
            let value = if let Ok(Some(v)) = row.try_get::<Option<String>, _>(i) {
                AnyValue::StringOwned(v.into()) // Use StringOwned instead of String
            } else if let Ok(None) = row.try_get::<Option<String>, _>(i) {
                AnyValue::Null
            } else if let Ok(Some(v)) = row.try_get::<Option<i32>, _>(i) {
                AnyValue::Int32(v)
            } else if let Ok(None) = row.try_get::<Option<i32>, _>(i) {
                AnyValue::Null
            } else if let Ok(Some(v)) = row.try_get::<Option<i64>, _>(i) {
                AnyValue::Int64(v)
            } else if let Ok(None) = row.try_get::<Option<i64>, _>(i) {
                AnyValue::Null
            } else if let Ok(Some(v)) = row.try_get::<Option<f64>, _>(i) {
                AnyValue::Float64(v)
            } else if let Ok(None) = row.try_get::<Option<f64>, _>(i) {
                AnyValue::Null
            } else if let Ok(Some(v)) = row.try_get::<Option<f32>, _>(i) {
                AnyValue::Float32(v)
            } else if let Ok(None) = row.try_get::<Option<f32>, _>(i) {
                AnyValue::Null
            } else if let Ok(Some(v)) = row.try_get::<Option<bool>, _>(i) {
                AnyValue::Boolean(v)
            } else if let Ok(None) = row.try_get::<Option<bool>, _>(i) {
                AnyValue::Null
            } else {
                // Fallback to null
                AnyValue::Null
            };

            col_vec.push(value);
        }
    }

    // Convert to Polars Series and then to Columns
    let mut columns_vec: Vec<polars::prelude::Column> = Vec::new();
    for col_name in column_names {
        let col_data = columns_data.remove(&col_name).unwrap();
        let series = Series::from_any_values(col_name.as_str().into(), &col_data, true)
            .map_err(|e| format!("Failed to create series for column '{}': {}", col_name, e))?;
        columns_vec.push(series.into()); // Convert Series to Column
    }

    DataFrame::new(columns_vec).map_err(|e| format!("Failed to create DataFrame: {}", e))
}

pub fn df_to_json(df: &mut DataFrame) -> Result<Value, String> {
    let mut buf = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut buf);

    JsonWriter::new(&mut cursor)
        .with_json_format(JsonFormat::Json)
        .finish(df)
        .map_err(|e| format!("Polars JSON write error: {}", e))?;

    let json_str = String::from_utf8(buf).map_err(|e| format!("UTF-8 conversion error: {}", e))?;
    serde_json::from_str(&json_str).map_err(|e| e.to_string())
}
