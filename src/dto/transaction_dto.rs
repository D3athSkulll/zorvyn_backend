use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTransactionRequest{
    pub amount: f64,
    pub r#type: String,
    pub category: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct TransactionQuery {
    pub r#type: Option<String>,
    pub category: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}