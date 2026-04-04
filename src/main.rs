use axum ::{routing::get, Router, extract::State};
mod config;
use config::db::connect_db;
use config::state::AppState;

#[tokio::main]
async fn main(){
    let pool = connect_db().await;
    println!("DB connected!");

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/", get(root))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1::3000");

    axum::serve(listener, app).await.unwrap();
}

async fn root(State(state): State<AppState>)-> String{
    format!("DB pool size: {}", state.db.size())
}