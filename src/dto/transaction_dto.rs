use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTransactionRequest{
    pub amount: f64,
    pub r#type: String,
    pub category: String,
    pub description: Option<String>,
}