use axum::{Router, routing::{post, get, put, delete}, middleware};
use crate::{
    config::state::AppState,
    handlers::transaction_handler::{create_tx_handler, get_tx_handler, update_tx_handler, delete_tx_handler},
    middlewares::{
        auth::auth_middleware,
        role::require_roles
    }
};

pub fn transaction_routes()-> Router<AppState>{
    // Public : Viewer (GET only)
    let public_routes = Router::new()
        .route("/transactions", get(get_tx_handler))
        .layer(middleware::from_fn(require_roles(vec!["viewer", "analyst", "admin"])));

    // Protected : Analyst (POST + PUT + Delete Own) + Admin (Delete Any)
    let protected_routes = Router::new()
        .route("/transactions", post(create_tx_handler))
        .route("/transactions/{id}", put(update_tx_handler))
        .route("/transactions/{id}", delete(delete_tx_handler))
        .layer(middleware::from_fn(require_roles(vec!["analyst", "admin"])));


    public_routes
        .merge(protected_routes)
        .layer(middleware::from_fn(auth_middleware))
}