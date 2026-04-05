use axum::{
    extract::{State, Json, Request},
    http::StatusCode,
};

use serde_json::json;

use crate::{
    config::state::AppState,
    dto::user_dto::{CreateUserRequest, LoginRequest},
    repositories::user_repo::{create_user, find_user_by_email},
    utils::{
        hash::{
            hash_password,
            verify_password,
        },
        jwt::{
            generate_token,
            Claims,
        },
        response::{error,success, success_with_message},
    },
};

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) ->Result<Json<serde_json::Value>,(StatusCode,String)> {
    
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
        Ok(user) => Ok(Json(success(json!({
            "id": user.id,
            "email": user.email,
            "role": user.role,
        })))),

        Err(e) => {
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.constraint() == Some("users_email_key") {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        error("Email already exists").to_string()
                    ));
                }
            }

            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                error("Something went wrong").to_string()
            ))
        }
    }
}

pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {

    let user = match find_user_by_email(&state.db, &payload.email).await{
        Ok(u)=>u,
        Err(_)=>{
            return Err((
                StatusCode::UNAUTHORIZED,
                error("Invalid credentials").to_string()
            ))
        }
    };

    let is_valid = verify_password(&user.password_hash, &payload.password);

    if !is_valid{
        return Err((
            StatusCode::UNAUTHORIZED,
            error("Invalid credentials").to_string()
        ))
    };

    let token = generate_token(&user.id.to_string(), &user.email, &user.role);

    Ok(Json(success(json!({
        "token": token,
    }))))

}

pub async fn protected_route(
    req: Request,
) -> Result<String, StatusCode> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    Ok(format!("Hello user {}", claims.email))
}

pub async fn admin_only() -> &'static str {
    "Welcome Admin !!"
}
