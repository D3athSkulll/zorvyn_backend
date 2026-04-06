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

pub async fn update_transaction(
    pool: &PgPool,
    tx_id: Uuid,
    user_id: Uuid,
    amount: f64,
    t_type: &str,
    category: &str,
    description: Option<&str>
)-> Result<Transaction, sqlx::Error>{
    
    let tx = sqlx::query_as::<_,Transaction>(
        r#"
        UPDATE transactions
        SET amount = $1,
            type = $2,
            category = $3,
            description = $4
        WHERE id = $5 AND user_id = $6
        RETURNING *
        "#
    )
    .bind(amount)
    .bind(t_type)
    .bind(category)
    .bind(description)
    .bind(tx_id)
    .bind(user_id)
    .fetch_optional(pool)
    .await?;

    match tx{
        Some(t)=> Ok(t),
        None => Err(sqlx::Error::RowNotFound)
    }

}

pub async fn delete_transaction(
    pool: &PgPool,
    tx_id: Uuid,
    user_id: Uuid,
    role: &str,
) -> Result<Transaction, sqlx::Error> {

    let delete_query = if role == "admin"{
        sqlx::query_as::<_, Transaction>(r#"
        DELETE FROM transactions
        WHERE id = $1
        RETURNING *
        "#)
        .bind(tx_id)
    }else{
        //Analyst
        sqlx::query_as::<_, Transaction>(
        r#"
        DELETE FROM transactions
        WHERE id = $1 AND user_id = $2
        RETURNING *
        "#)
        .bind(tx_id)
        .bind(user_id)
    };

    let tx = delete_query
    .fetch_optional(pool)
    .await?;

    match tx {
        Some(t) => Ok(t),
        None => Err(sqlx::Error::RowNotFound),
    }
}