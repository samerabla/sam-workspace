use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AttributeValue {
    String(String),
    Integer(i64),
    Decimal(Decimal),
    Bool(bool),
    List(Vec<AttributeValue>),
    DateTime(OffsetDateTime),
}

impl AttributeValue {
    pub fn as_string(&self) -> Option<&String> {
        match self {
            AttributeValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<i64> {
        match self {
            AttributeValue::Integer(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_decimal(&self) -> Option<Decimal> {
        match self {
            AttributeValue::Decimal(d) => Some(*d),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            AttributeValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_date_time(&self) -> Option<time::OffsetDateTime> {
        match self {
            AttributeValue::DateTime(dt) => Some(*dt),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&Vec<AttributeValue>> {
        match self {
            AttributeValue::List(l) => Some(l),
            _ => None,
        }
    }
}

impl From<Value> for AttributeValue {
    fn from(value: Value) -> Self {
        match value {
            Value::String(s) => {
                if let Ok(dt) =
                    OffsetDateTime::parse(&s, &time::format_description::well_known::Rfc3339)
                {
                    AttributeValue::DateTime(dt)
                } else {
                    AttributeValue::String(s)
                }
            }
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    AttributeValue::Integer(i)
                } else if let Some(d) = n.as_f64() {
                    AttributeValue::Decimal(Decimal::from_f64_retain(d).unwrap_or(Decimal::ZERO))
                } else {
                    AttributeValue::String(n.to_string())
                }
            }
            Value::Bool(b) => AttributeValue::Bool(b),
            Value::Array(arr) => {
                AttributeValue::List(arr.into_iter().map(AttributeValue::from).collect())
            }
            _ => AttributeValue::String(value.to_string()),
        }
    }
}

impl From<AttributeValue> for Value {
    fn from(val: AttributeValue) -> Self {
        match val {
            AttributeValue::String(s) => Value::String(s),
            AttributeValue::Integer(n) => Value::Number(n.into()),
            AttributeValue::Decimal(d) => {
                // Convert Decimal to f64, then to serde_json::Number
                serde_json::Number::from_f64(d.to_f64().unwrap_or(0.0))
                    .map(Value::Number)
                    .unwrap_or(Value::Null)
            }
            AttributeValue::Bool(b) => Value::Bool(b),
            AttributeValue::List(l) => Value::Array(l.into_iter().map(Value::from).collect()),
            AttributeValue::DateTime(dt) => {
                // Convert DateTime to RFC3339 string
                Value::String(
                    dt.format(&time::format_description::well_known::Rfc3339)
                        .unwrap_or_default(),
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AttributeMap(pub HashMap<String, AttributeValue>);

impl From<Value> for AttributeMap {
    fn from(value: Value) -> Self {
        let mut map = HashMap::new();

        if let Value::Object(obj) = value {
            for (k, v) in obj {
                match v {
                    Value::String(s) => {
                        // Check if the string can be parsed as a DateTime
                        if let Ok(dt) = OffsetDateTime::parse(
                            &s,
                            &time::format_description::well_known::Rfc3339,
                        ) {
                            map.insert(k.clone(), AttributeValue::DateTime(dt));
                        } else {
                            map.insert(k.clone(), AttributeValue::String(s.clone()));
                        }
                        map.insert(k, AttributeValue::String(s));
                    }
                    Value::Number(n) => {
                        if let Some(num) = n.as_i64() {
                            map.insert(k, AttributeValue::Integer(num));
                        } else if let Some(num) = n.as_f64() {
                            map.insert(
                                k,
                                AttributeValue::Decimal(
                                    Decimal::from_f64_retain(num).unwrap_or(Decimal::ZERO),
                                ),
                            );
                        } else {
                            map.insert(k, AttributeValue::String(n.to_string()));
                        }
                    }
                    Value::Bool(b) => {
                        map.insert(k, AttributeValue::Bool(b));
                    }
                    Value::Array(arr) => {
                        let list = arr.into_iter().map(AttributeValue::from).collect();
                        map.insert(k, AttributeValue::List(list));
                    }
                    _ => {} // Skip other JSON types
                }
            }
        }

        AttributeMap(map)
    }
}

impl From<AttributeMap> for Value {
    fn from(map: AttributeMap) -> Value {
        let mut json_map = serde_json::Map::new();

        for (k, v) in map.0 {
            match v {
                AttributeValue::String(s) => {
                    json_map.insert(k, serde_json::Value::String(s));
                }
                AttributeValue::Integer(n) => {
                    json_map.insert(k, serde_json::Value::Number(n.into()));
                }
                AttributeValue::Decimal(d) => {
                    let n = serde_json::Number::from_f64(d.to_f64().unwrap_or(0.0))
                        .map(Value::Number)
                        .unwrap_or(Value::Null);
                    json_map.insert(k, n);
                }
                AttributeValue::Bool(b) => {
                    json_map.insert(k, serde_json::Value::Bool(b));
                }
                AttributeValue::List(l) => {
                    let arr: Vec<Value> = l.into_iter().map(Value::from).collect();
                    json_map.insert(k, serde_json::Value::Array(arr));
                }
                AttributeValue::DateTime(dt) => {
                    // Convert DateTime to RFC3339 string
                    let dt_str = dt
                        .format(&time::format_description::well_known::Rfc3339)
                        .unwrap_or_default();
                    json_map.insert(k, serde_json::Value::String(dt_str));
                }
            }
        }

        serde_json::Value::Object(json_map)
    }
}

pub enum ResourceType {
    User,
    Category,
    Listing,
    Field,
    FieldOption,
    DashNavItem,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    pub resource_type: String,
    pub attributes: HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub attributes: HashMap<String, AttributeValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Context {
    pub ip_address: String,
    pub attributes: HashMap<String, AttributeValue>,
}
