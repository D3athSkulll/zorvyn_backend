use axum ::{Router, extract::State};

mod config;
mod repositories;
mod models;
mod routes;
mod handlers;
mod dto;
mod utils;
mod middlewares;

use config::db::connect_db;
use config::state::AppState;
use repositories::user_repo::create_user;
use routes::user_routes::user_routes;
use routes::transaction_routes::transaction_routes;

#[tokio::main]
async fn main(){
    let pool = connect_db().await;
    println!("DB connected!");

    let state = AppState { db: pool };

    let app = Router::new()
        .merge(user_routes())
        .merge(transaction_routes())
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1::3000");

    axum::serve(listener, app).await.unwrap();

    let user = create_user(
    &state.db,
    "Test",
    "test@example.com",
    "hashed_password",
    "admin"
    ).await.unwrap();

    println!("User created: {:?}", user);
}

async fn root(State(state): State<AppState>)-> String{
    format!("DB pool size: {}", state.db.size())
}