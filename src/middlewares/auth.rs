use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response
};
use serde_json::json;

use crate::utils::{
    jwt::decode_token,
    app_error::AppError,
};

pub async fn auth_middleware(
    mut req: Request,
    next: Next
)-> Result<Response, AppError> {

    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let token = match auth_header{
        Some(header) if header.starts_with("Bearer ")=>{
            header.trim_start_matches("Bearer ").to_string()
        }
        _=>return Err(AppError {
                status: StatusCode::UNAUTHORIZED,
                body: json!({
                    "success": false,
                    "message": "Missing or invalid Authorization header"
                }),
            })
    };
    
    let claims = match decode_token(&token){
        Ok(c)=>c,
        Err(_) => return Err(AppError {
                status: StatusCode::UNAUTHORIZED,
                body: json!({
                    "success": false,
                    "message": "Invalid Token"
                }),
            })
    };

    //attaching claims to req to be used later
    req.extensions_mut().insert(claims);
    
    Ok(next.run(req).await)
}