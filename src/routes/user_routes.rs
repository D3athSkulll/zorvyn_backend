use axum::{Router, routing::{post, get}, middleware};
use crate::{
    config::state::AppState,
    handlers::user_handler::{register_user, login_user, protected_route, admin_only},
    middlewares::{
        auth::auth_middleware,
        role::require_role,
    }
};

pub fn user_routes()-> Router<AppState>{
    let public_routes = Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user));

    let protected_routes = Router::new()
        .route("/me", get(protected_route))
        .route("/admin", get(admin_only))
        .layer(middleware::from_fn(auth_middleware));

    let admin_routes = Router::new()
        .route("/admin-only", get(admin_only))
        .layer(middleware::from_fn(require_role("admin")))
        .layer(middleware::from_fn(auth_middleware));

    public_routes
        .merge(protected_routes)
        .merge(admin_routes)
    }