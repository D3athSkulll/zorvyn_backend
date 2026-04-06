use axum ::{Router,routing::{get}, extract::State};

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
use routes::user_routes::user_routes;
use routes::transaction_routes::transaction_routes;
use routes::dashboard_routes::dashboard_routes;

#[tokio::main]
async fn main(){
    let pool = connect_db().await;
    println!("DB connected!");

    let state = AppState { db: pool };

    let app = Router::new()
        .route("/", get(root))
        .merge(user_routes())
        .merge(transaction_routes())
        .merge(dashboard_routes())
        .with_state(state.clone());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();

}

async fn root(State(state): State<AppState>)-> String{
    format!(
r#"
🚀 Zorvyn Finance Backend is running!

📡 Server Info:
- DB Pool Size: {}

🔐 Auth Routes:
POST   /register        → Create user
POST   /login           → Login and get JWT

👤 User (Admin + Analyst Only):
GET    /users           → List users
PATCH  /users/:id       → Update user role
DELETE /users/:id       → Delete user

💰 Transactions:
POST   /transactions            → Create transaction
GET    /transactions            → Get transactions (filters + pagination)
PUT    /transactions/:id        → Update transaction
DELETE /transactions/:id        → Delete transaction

📊 Dashboard:
GET    /dashboard       → Financial summary + trends

🧪 Example:
GET /transactions?limit=5&offset=0
Authorization: Bearer <your_token>

⚠️ Notes:
- Use Bearer token for protected routes
- Roles: viewer | analyst | admin
- Analyst → cannot delete others' data
- Admin → full access

"#,
        state.db.size()
    )
}