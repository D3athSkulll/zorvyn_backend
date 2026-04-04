use axum::{Router, routing::{post, get}, middleware};
use crate::{
    config::state::AppState,
    handlers::transaction_handler::{create_tx, get_transactions},
    middlewares::{
        auth::auth_middleware,
    }
};

pub fn transaction_routes()-> Router<AppState>{
    Router::new()
        .route("/transactions", post(create_tx).get(get_transactions))
        .layer(middleware::from_fn(auth_middleware))
}