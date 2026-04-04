use axum::{
    extract::{State, Json},
    http::StatusCode,
};

use serde_json::json;

use crate::{
    config::state::AppState,
    dto::user_dto::CreateUserRequest,
    repositories::user_repo::create_user,
    utils::hash::hash_password,
};

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    
    let hashed_password = hash_password(&payload.password);

    let result = create_user(
        &state.db,
        &payload.name,
        &payload.email,
        &hashed_password, // not hashed
        &payload.role,
    )
    .await;
    
    match result {
        Ok(user) => Ok(Json(json!({
            "success": true,
            "data": {
                "id": user.id,
                "email": user.email,
                "role": user.role
            }
        }))),

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
