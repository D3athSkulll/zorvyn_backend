use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Transaction{
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub r#type: String, // type is keyword
    pub category: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}