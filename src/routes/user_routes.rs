use axum::{Router, routing::{post, get}, middleware};
use crate::{
    config::state::AppState,
    handlers::user_handler::{register_user, login_user, protected_route},
    middlewares::auth::auth_middleware,
};

pub fn user_routes()-> Router<AppState>{
    let public_routes = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user));

    let protected_routes = Router::new()
        .route("/me", get(protected_route))
        .layer(middleware::from_fn(auth_middleware));

    public_routes.merge(protected_routes)
}