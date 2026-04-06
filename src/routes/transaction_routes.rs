use axum::{Router, routing::{post, get, put, delete}, middleware};
use crate::{
    config::state::AppState,
    handlers::transaction_handler::{create_tx, get_transactions, update_tx, delete_tx},
    middlewares::auth::auth_middleware
};

pub fn transaction_routes()-> Router<AppState>{
    Router::new()
        .route("/transactions", post(create_tx).get(get_transactions))
        .route("/transactions/{id}", put(update_tx))
        .route("/transactions/{id}", delete(delete_tx))
        .layer(middleware::from_fn(auth_middleware))
}