use axum::{Router, routing::get, middleware};

use crate::{
    config::state::AppState,
    handlers::dashboard_handler::get_dashboard,
    middlewares::auth::auth_middleware,
};

pub fn dashboard_routes() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(get_dashboard))
        .layer(middleware::from_fn(auth_middleware))
}