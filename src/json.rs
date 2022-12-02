use actix_web::error::{InternalError, JsonPayloadError, PayloadError};
use actix_web::HttpResponse;
use actix_web::web::JsonConfig;
use serde_json::Value;
use std::collections::HashMap;

use crate::Payload;

/// Check if serde_json::Value is empty
///
/// Example
/// ```
/// use serde_json::json;
///
/// use library::json::is_empty;
///
/// fn main() {
///     let object = json!({ "A": 65, "B": 66, "C": "test", "D": "" });
///     let b = is_empty(&object);
/// }
/// ```
pub fn is_empty<T: serde::Serialize>(item: &T) -> bool {
    let result = serde_json::to_string(item);
    if result.is_ok() {
        let bindings = result.unwrap();
        let map:HashMap<String, Value> = serde_json::from_str(&bindings).unwrap();

        for (_, value) in map.iter() {
            match () {
                _ if value.is_array() && value.as_array().is_some() && value.as_array().unwrap().len() > 0 => return false,
                _ if value.is_string() && value.as_str().is_some() && !value.as_str().unwrap().trim().is_empty() => return false,
                _ if value.is_object() && value.as_object().is_some() => return false,
                _ if value.is_boolean() && value.as_bool().is_some() => return false,
                _ if value.is_i64() && value.as_i64().is_some() => return false,
                _ if value.is_f64() && value.as_f64().is_some() => return false,
                _ if value.is_u64() && value.as_u64().is_some() => return false,
                _ => {}
            }
        }
    }

    true
}

/// Normalize field
pub fn normalize<T>(item: T) -> T
    where T: Clone + serde::Serialize + serde::de::DeserializeOwned
{
    let bindings = item.clone();
    let result = serde_json::to_string(&bindings);
    if result.is_ok() {
        // Set bindings
        let bindings = result.unwrap();
        let map:HashMap<String, Value> = serde_json::from_str(&bindings).unwrap();

        // Create new map and then loop current map
        let mut items:HashMap<String, Value> = HashMap::new();
        for (key, value) in map.iter() {
            // Set key & value
            let key = key.clone();
            let value = value.clone();

            // Match type
            match () {
                _ if value.is_array() => { items.insert(key, normalize_array(value.clone())); },
                _ if value.is_boolean() => { items.insert(key, normalize_bool(value.clone())); },
                _ if value.is_string() => { items.insert(key, normalize_string(value.clone())); },
                _ if value.is_f64() => { items.insert(key, normalize_f64(value.clone())); },
                _ if value.is_i64() => { items.insert(key, normalize_i64(value.clone())); },
                _ if value.is_u64() => { items.insert(key, normalize_u64(value.clone())); },
                _ if value.is_object() => { items.insert(key, normalize_object(value.clone())); },
                _ => { items.insert(key, Value::Null); }
            }
        }

        // Return value to custom struct
        return match serde_json::to_value(items) {
            Ok(i) => {
                match serde_json::from_value(i) {
                    Ok(i) => i,
                    Err(_) => item
                }
            },
            Err(_) => item
        };
    }

    item
}

// Normalize array type serde_json::Value
fn normalize_array(item: Value) -> Value {
    // Return item value
    return match item.as_array().is_some() && item.as_array().unwrap().len() > 0 {
        true => {
            // Set initial items
            let mut items:Vec<Value> = Vec::new();
            let vectors = item.as_array().unwrap().clone();

            // Loop through vectors
            for value in vectors.iter() {
                let i = normalize(value.clone());
                if !i.is_null() {
                    items.push(i.clone());
                }
            }

            // Check items length
            return match items.len() > 0 {
                true => {
                    // Return value
                    match serde_json::to_string(&items) {
                        Ok(i) => serde_json::from_str(&i).unwrap(),
                        Err(_) => Value::Null
                    }
                },
                false => Value::Null
            };
        },
        false => Value::Null
    }
}

// Normalize bool type serde_json::Value
fn normalize_bool(item: Value) -> Value {
    // Return item value
    return match item.as_bool().is_some() {
        true => item.clone(),
        false => Value::Null,
    }
}

// Normalize f64 type serde_json::Value
fn normalize_f64(item: Value) -> Value {
    // Return item value
    return match item.as_f64().is_some() {
        true => item.clone(),
        false => Value::Null,
    };
}

// Normalize i64 type serde_json::Value
fn normalize_i64(item: Value) -> Value {
    // Return item value
    return match item.as_i64().is_some() {
        true => item.clone(),
        false => Value::Null,
    };
}

// Normalize u64 type serde_json::Value
fn normalize_u64(item: Value) -> Value {
    // Return item value
    return match item.as_u64().is_some() {
        true => item.clone(),
        false => Value::Null,
    };
}

// Normalize object type serde_json::Value
fn normalize_object(item: Value) -> Value {
    // Return item value
    return match () {
        _ if item.is_array() => normalize_array(item),
        _ if item.is_boolean() => normalize_bool(item.clone()),
        _ if item.is_string() => normalize_string(item.clone()),
        _ if item.is_f64() => normalize_f64(item.clone()),
        _ if item.is_i64() => normalize_i64(item.clone()),
        _ if item.is_u64() => normalize_u64(item.clone()),
        _ if item.as_object().is_some() => {
            // Create new items
            let mut items:HashMap<String, Value> = HashMap::new();
            let result = item.as_object().unwrap().clone();
            for (key, value) in result {
                let i = normalize_object(value.clone());
                items.insert(key, i.clone());
            }

            match items.len() > 0 {
                true => {
                    match serde_json::to_string(&items) {
                        Ok(i) => serde_json::from_str(&i).unwrap(),
                        Err(_) => Value::Null
                    }
                },
                false => Value::Null
            }
        },
        _ => Value::Null
    };
}

// Normalize string type serde_json::Value
fn normalize_string(item: Value) -> Value {
    // Return item value
    return match () {
        _ if item.as_str().is_some() && !item.as_str().unwrap().trim().is_empty() => Value::String(item.as_str().unwrap().trim().to_string()),
        _ => Value::Null,
    }
}

/// Create staging for json config
pub fn stage(json_limit: usize) -> JsonConfig {
    JsonConfig::default()
        .limit(json_limit)
        .error_handler(|err, _req| {
            // Create new json response
            let mut response = Payload::new(400);

            // Match error
            match err {
                JsonPayloadError::ContentType => response.error = String::from("Invalid Content-Type header"),
                JsonPayloadError::Deserialize(error) => response.error = format!("Json deserialize error: {}", error.to_string()),
                JsonPayloadError::Payload(error) => {
                    match error {
                        PayloadError::Incomplete(error) => response.error = format!("A payload reached EOF, but is not complete. With error: {}", error.unwrap().to_string()),
                        PayloadError::EncodingCorrupted => response.error = String::from("Can not decode content-encoding"),
                        PayloadError::Overflow => response.error = String::from("Json payload size is bigger than allowed"),
                        PayloadError::UnknownLength => response.error = String::from("A payload length is unknown"),
                        PayloadError::Http2Payload(error) => response.error = error.to_string(),
                        PayloadError::Io(error) => response.error = error.to_string(),
                        _ => response.error = String::from("An error occurred while processing your request"),
                    }
                },
                _ => response.error = String::from("An error occurred while processing your request"),
            }

            InternalError::from_response(
                JsonPayloadError::ContentType,
                HttpResponse::BadRequest().json(response)
            ).into()
        })
}