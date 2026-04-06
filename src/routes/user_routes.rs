use axum::{Router, routing::{post, get, patch, delete}, middleware};
use crate::{
    config::state::AppState,
    handlers::user_handler::{create_user_handler, login_user_handler, protected_route_handler, admin_only, list_users_handler, delete_user_handler, update_user_role_handler},
    middlewares::{
        auth::auth_middleware,
        role::require_roles,
    }
};

pub fn user_routes()-> Router<AppState>{
    let public_routes = Router::new()
        .route("/register", post(create_user_handler))
        .route("/login", post(login_user_handler));

    let protected_routes = Router::new()
        .route("/me", get(protected_route_handler))
        .route("/admin", get(admin_only))
        .layer(middleware::from_fn(auth_middleware));

    let admin_routes = Router::new()
        .route("/users", get(list_users_handler))
        .route("/users/{id}", patch(update_user_role_handler))
        .route("/users/{id}", delete(delete_user_handler))
        .layer(middleware::from_fn(require_roles(vec!["admin"])))
        .layer(middleware::from_fn(auth_middleware));
        

    public_routes
        .merge(protected_routes)
        .merge(admin_routes)
    }