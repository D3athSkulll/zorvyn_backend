use validator::ValidationErrors;
use serde_json::{json, Value};

pub fn format_validation_errors(errors: ValidationErrors) -> Value {
    let mut error_map = serde_json::Map::new();

    for (field, field_errors) in errors.field_errors() {
        let messages: Vec<String> = field_errors
            .iter()
            .map(|err| {
                err.message
                    .clone()
                    .unwrap_or_else(|| "Invalid value".into())
                    .to_string()
            })
            .collect();

        error_map.insert(field.to_string(), json!(messages));
    }

    json!({
        "success": false,
        "errors": error_map
    })
}