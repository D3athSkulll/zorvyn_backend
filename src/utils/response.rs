use serde_json::{json, Value};

pub fn success(data: Value) -> Value {
    json!({
        "success": true,
        "data": data
    })
}

pub fn success_with_message(data: Value, message: &str) -> Value {
    json!({
        "success": true,
        "data": data,
        "message": message
    })
}

pub fn error(message: &str) -> Value {
    json!({
        "success": false,
        "message": message
    })
}