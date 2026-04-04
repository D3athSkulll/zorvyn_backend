use sqlx::PgPool;
use crate::models::transaction::Transaction;
use uuid::Uuid;

pub async fn create_transaction(
    pool: &PgPool,
    user_id: Uuid,
    amount: f64,
    t_type: &str,
    category: &str,
    description: Option<&str>      
)-> Result<Transaction, sqlx::Error>{

    let tx = sqlx::query_as::<_,Transaction>(
        r#"
        INSERT INTO transactions (user_id, amount, type, category, description)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING *
        "#
    )
    .bind(user_id)
    .bind(amount)
    .bind(t_type)
    .bind(category)
    .bind(description)
    .fetch_one(pool)
    .await?;

    Ok(tx)
}