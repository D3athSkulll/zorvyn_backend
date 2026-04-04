use axum::{
    extract::{State, Json, Extension},
    http::StatusCode,
};
use uuid::Uuid;

use crate::{
    config::state::AppState,
    dto::transaction_dto::CreateTransactionRequest,
    models::transaction::Transaction,
    repositories::transaction_repo::create_transaction,
    utils::jwt::Claims,
};

use serde_json::json;

pub async fn create_tx(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateTransactionRequest>,
)-> Result<Json<serde_json::Value>, (StatusCode, String)>{

    let user_id = match claims.sub.parse::<Uuid>(){
        Ok(id)=>id,
        Err(_) => return Err((StatusCode::UNAUTHORIZED, "Invalid token".to_string())),
    };

    let result = create_transaction(
        &state.db,
        user_id,
        payload.amount,
        &payload.r#type,
        &payload.category,
        payload.description.as_deref(),
    )
    .await;

    match result{
        Ok(transaction)=>Ok(Json(json!({
        "success": true,
        "data": {
            "id":transaction.id,
            "amount": transaction.amount,
            "user_id": transaction.user_id,
            "category": transaction.category,
        },
    })))    ,

        Err(e) => {
            println!("DB error: {:?}", e);

            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code() == Some("23503".into()) {
                    return Err((StatusCode::BAD_REQUEST, "Invalid user".to_string()));
                }
            }

            Err((StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong".to_string()))
        }
}
}

pub async fn get_transactions(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
)-> Json<serde_json::Value>{
    let user_id: Uuid = claims.sub.parse().unwrap();

    let txs = sqlx::query_as::<_, Transaction>(
        "SELECT * FROM transactions WHERE user_id = $1 ORDER BY created_at DESC"
    )
    .bind(user_id)
    .fetch_all(&state.db)
    .await
    .unwrap();

    Json(json!({
        "success": true,
        "data": txs
    }))
}