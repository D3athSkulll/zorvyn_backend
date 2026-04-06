use axum::{
    extract::{State, Json, Extension, Query, Path},
    http::StatusCode,
};
use uuid::Uuid;
use sqlx::QueryBuilder;
use serde_json::{json,Value};
use validator::Validate;

use crate::{
    config::state::AppState,
    dto::transaction_dto::{CreateTransactionRequest, TransactionQuery, UpdateTransactionRequest},
    models::transaction::Transaction,
    repositories::transaction_repo::{create_transaction, update_transaction, delete_transaction},
    utils::{
        app_error::AppError,
        jwt::Claims,
        response::{error,success,success_with_message},
        validation::format_validation_errors
    }
};

pub async fn create_tx(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateTransactionRequest>,
)-> Result<Json<Value>, AppError>{
    
    if let Err(e) = payload.validate() {
        return Err(AppError {
            status: StatusCode::BAD_REQUEST,
            body: format_validation_errors(e),
        });
    }

    let user_id = match claims.sub.parse::<Uuid>(){
        Ok(id)=>id,
        Err(_) => return Err(AppError{
            status: StatusCode::UNAUTHORIZED,
            body: json!({
                    "success": false,
                    "message": "Invalid Token"
                }) 
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
                        body: json!({
                            "success": false,
                            "message": e.to_string()
                        })
                    });
                }
            }

            Err(AppError{
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: json!({
                        "success": false,
                        "message": "Invalid Token"
                    }) 
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
            body: json!({
                    "success": false,
                    "message": "Invalid Token"
            }),
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
                body: json!({
                    "success": false,
                    "message": "Invalid Transaction Type"
                })
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
                body: json!({
                    "success": false,
                    "message": "Something Went Wrong"
                })
            })
        }

    }
}

pub async fn update_tx(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(tx_id): Path<Uuid>,
    Json(payload): Json<UpdateTransactionRequest>,
)-> Result<Json<Value>, AppError>{

    payload.validate().map_err(|errors| AppError {
        status: StatusCode::BAD_REQUEST,
        body: json!({
            "success": false,
            "message": "Invalid input",
            "errors": errors.to_string()
        }),
    })?;

    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| AppError {
            status: StatusCode::UNAUTHORIZED,
            body: json!({
                "success": false,
                "message": "Invalid token"
            }),
        })?;


    let result = update_transaction(
        &state.db,
        tx_id,
        user_id,
        payload.amount,
        &payload.r#type,
        &payload.category,
        payload.description.as_deref(),
    )
    .await;

    match result{
        Ok(tx)=> Ok(Json(success(json!(tx)))),
        Err(e)=> if let sqlx::Error::RowNotFound = e {
            Err(AppError{
                status: StatusCode::NOT_FOUND,
                body: json!({
                    "success": false,
                    "message": "Transaction Not Found or not owned by user"
                })
            })
        } else{
            Err(AppError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: json!({
                    "success": false,
                    "message": format!("{:?}", e)
                })
            })
        },
    }
}

pub async fn delete_tx(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(tx_id): Path<Uuid>,
)-> Result<Json<Value>, AppError>{

    if claims.role != "admin"{
        return Err(AppError{
            status: StatusCode::FORBIDDEN,
            body: json!({
                "status": false,
                "message": "You are not allowed to delete transactions."
            })
        })
    }

    let user_id = claims.sub.parse::<Uuid>()
        .map_err(|_| AppError {
            status: StatusCode::UNAUTHORIZED,
            body: json!({
                "success": false,
                "message": "Invalid token"
            }),
        })?;

    let result = delete_transaction(
        &state.db,
        tx_id,
        user_id
    )
    .await;

    match result{
        Ok(tx)=> Ok(Json(success(json!({
            "transaction": tx,
            "message": "Transaction deleted successfully"
        })))),
        Err(e)=> if let sqlx::Error::RowNotFound=e{
            Err(AppError{
                status: StatusCode::NOT_FOUND,
                body: json!({
                    "status": false,
                    "message": "Transaction not found or not owned by the user"
                }),
            })
        } else{
            Err(AppError{
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: json!({
                    "status": false,
                    "message": "Failed to delete transaction"
                })
            })
        }
    }
    
}