use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use crate::{
    config::state::AppState,
    dto::user_dto::CreateUserRequest,
    repositories::user_repo::create_user,
};

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<String, (StatusCode, String)> {
    let result = create_user(
        &state.db,
        &payload.name,
        &payload.email,
        &payload.password, // not hashed
        &payload.role,
    )
    .await;
    
    match result {
        Ok(user) => Ok(format!("User created: {}", user.id)),

        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.constraint() == Some("users_email_key") {
                    return Err((StatusCode::BAD_REQUEST, "Email already exists".to_string()));
                }
            }

            Err((StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong".to_string()))
        }
    }
}
