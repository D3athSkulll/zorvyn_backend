use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response
};

use crate::utils::jwt::decode_token;

pub async fn auth_middleware(
    mut req: Request,
    next: Next
)-> Result<Response, StatusCode> {

    let auth_header = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    let token = match auth_header{
        Some(header) if header.starts_with("Bearer ")=>{
            header.trim_start_matches("Bearer ").to_string()
        }
        _=>return Err(StatusCode::UNAUTHORIZED),
    };
    
    let claims = match decode_token(&token){
        Ok(c)=>c,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    //attaching claims to req to be used later
    req.extensions_mut().insert(claims);
    
    Ok(next.run(req).await)
}