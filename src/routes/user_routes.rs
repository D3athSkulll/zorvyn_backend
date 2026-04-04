use axum::{Router, routing::post};
use crate::{
    config::state::AppState,
    handlers::user_handler::register_user,
};

pub fn user_routes()-> Router<AppState>{
    Router::new()
        .route("/register", post(register_user))
}