use axum::{
    extract::{Json, Path, Request, State},
    http::StatusCode,
};

use uuid::Uuid;
use serde_json::{json,Value};
use validator::Validate;

use crate::{
    config::state::AppState,
    dto::user_dto::{CreateUserRequest, LoginRequest, UpdateUserRoleRequest},
    repositories::user_repo::{create_user, delete_user, find_user_by_email, get_all_users, update_user_role},
    utils::{
        app_error::AppError, hash::{
            hash_password,
            verify_password,
        }, jwt::{
            Claims, generate_token
        }, response::{error,success, success_with_message}, validation::format_validation_errors
    },
};

pub async fn create_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) ->Result<Json<Value>,AppError> {

    if let Err(e) = payload.validate() {
        return Err(AppError {
            status: StatusCode::BAD_REQUEST,
            body: format_validation_errors(e),
        });
    }
    
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
                    return Err(AppError{
                        status: StatusCode::BAD_REQUEST,
                        body: json!({
                            "success": false,
                            "message": "Email already exists"    
                        })
                    });
                }
            }

            Err(AppError{
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: json!({
                        "success": false,
                        "message": "Something Went Wrong"    
                })
            })
        }
    }
}

pub async fn login_user_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<Value>, AppError> {

    let user = match find_user_by_email(&state.db, &payload.email).await{
        Ok(u)=>u,
        Err(_)=>{
            return Err(AppError{
                status: StatusCode::UNAUTHORIZED,
                body: json!({
                    "success": false,
                    "message": "Invalid Credentials"    
                })
            })
        }
    };

    let is_valid = verify_password(&user.password_hash, &payload.password);

    if !is_valid{
        return Err(AppError{
            status: StatusCode::UNAUTHORIZED,
            body: json!({
                    "success": false,
                    "message": "Invalid Credentials"    
            })
        })
    };

    let token = generate_token(&user.id.to_string(), &user.email, &user.role);

    Ok(Json(success(json!({
        "token": token,
    }))))

}

pub async fn list_users_handler(
    State(state): State<AppState>,
)-> Result<Json<Value>, AppError>{

    let result = get_all_users(&state.db)
        .await;

    match result{
        Ok(users)=>Ok(Json(success(json!(users)))),
        Err(e)=>Err(AppError{
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: json!({
                "status": false,
                "message": println!("{:?}",e),
            })
        })
    }
}

pub async fn update_user_role_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserRoleRequest>,
)-> Result<Json<Value>, AppError>{

    payload.validate().map_err(|_| AppError{
        status: StatusCode::BAD_REQUEST,
        body: json!({
            "success": false,
            "message": "Invalid role"
        })  
    })?;

    let result = update_user_role(&state.db, user_id, &payload.role)
    .await;

    match result{
        Ok(user)=> Ok(Json(success(json!(user)))),
        Err(e)=> if let sqlx::Error::RowNotFound = e {
                Err(AppError {
                    status: StatusCode::NOT_FOUND,
                    body: json!({
                        "success": false,
                        "message": "User not found"
                    }),
                })
            } else{
                Err(AppError{
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    body: json!({
                        "status": false,
                        "message": "Failed to update role"
                    })
                })
            }
    }
}

pub async fn delete_user_handler(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
)-> Result<Json<Value>, AppError>{
    
    let result = delete_user(&state.db, user_id)
    .await;

    match result{
        Ok(_)=>Ok(Json(success(json!({
            "message": "User Deleted Successfully"
        })))),
        Err(e)=> if let sqlx::Error::RowNotFound = e {
                Err(AppError {
                    status: StatusCode::NOT_FOUND,
                    body: json!({
                        "success": false,
                        "message": "User not found"
                    }),
                })
            } else{
                Err(AppError{
                    status: StatusCode::INTERNAL_SERVER_ERROR,
                    body: json!({
                        "status": false,
                        "message": "Failed to delete user"
                    })
                })
            }
    }
}

pub async fn protected_route_handler(
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

