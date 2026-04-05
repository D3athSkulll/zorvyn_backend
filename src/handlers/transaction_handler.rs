use axum::{
    extract::{State, Json, Extension, Query},
    http::StatusCode,
};
use uuid::Uuid;
use sqlx::QueryBuilder;
use serde_json::{json,Value};

use crate::{
    config::state::AppState,
    dto::transaction_dto::{CreateTransactionRequest, TransactionQuery},
    models::transaction::Transaction,
    repositories::transaction_repo::create_transaction,
    utils::{app_error::AppError, jwt::Claims, response::{error,success,success_with_message}}
};

pub async fn create_tx(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateTransactionRequest>,
)-> Result<Json<Value>, AppError>{

    let user_id = match claims.sub.parse::<Uuid>(){
        Ok(id)=>id,
        Err(_) => return Err(AppError{
            status: StatusCode::UNAUTHORIZED,
            message: "Invalid token".to_string()
        }),
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
        Ok(tx)=>Ok(Json(success(json!(tx)))),

        Err(e) => {
            println!("DB error: {:?}", e);

            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code() == Some("23503".into()) {
                    return Err(AppError{
                        status: StatusCode::BAD_REQUEST,
                        message: "Invalid user".to_string()
                    });
                }
            }

            Err(AppError{
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Something went wrong".to_string()
            })
        }
}
}

pub async fn get_transactions(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Query(params): Query<TransactionQuery>,
)-> Result<Json<Value>, AppError>{
    let user_id = match claims.sub.parse::<Uuid>(){
        Ok(id)=>id,
        Err(_) => return Err(AppError{
            status: StatusCode::UNAUTHORIZED,
            message: "Invalid token".to_string()
        }),
    };

    let mut qb = QueryBuilder::new(
        "SELECT * FROM transactions WHERE user_id = "
    );

    qb.push_bind(user_id);

    if let Some(ref tx_type) = params.r#type {
        if tx_type != "income" && tx_type != "expense" {
            return Err(AppError{
                status: StatusCode::BAD_REQUEST,
                message: "Invalid transaction type".to_string()
            });
        }
        qb.push(" AND type = ");
        qb.push_bind(tx_type);
    }

    if let Some(ref category) = params.category {
        qb.push(" AND category = ");
        qb.push_bind(category);
    }

    if let Some(start_date) = params.start_date {
        qb.push(" AND created_at >= ");
        qb.push_bind(start_date);
    }

    if let Some(end_date) = params.end_date {
        qb.push(" AND created_at <= ");
        qb.push_bind(end_date);
    }

    qb.push(" ORDER BY created_at DESC");

    let result = qb
        .build_query_as::<Transaction>()
        .fetch_all(&state.db)
        .await;

    match result{
        Ok(txs)=>Ok(Json(success(json!(txs)))),

        Err(e)=>{
            println!("DB error: {:?}", e);
            Err(AppError{
                status: StatusCode::INTERNAL_SERVER_ERROR,
                message: "Something went wrong".to_string(),
            })
        }

    }
}