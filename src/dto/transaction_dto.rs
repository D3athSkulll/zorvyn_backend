use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateTransactionRequest{
    #[validate(range(min = 0.01, message = "Amount must be positive"))]
    pub amount: f64,

    #[validate(custom(function = "validate_type"))]
    pub r#type: String,

    #[validate(custom(function = "validate_type"))]
    pub category: String,

    pub description: Option<String>,
}

fn validate_type(t: &str) -> Result<(), validator::ValidationError> {
    match t {
        "income" | "expense" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_type")),
    }
}

#[derive(Deserialize)]
pub struct TransactionQuery {
    pub r#type: Option<String>,
    pub category: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}