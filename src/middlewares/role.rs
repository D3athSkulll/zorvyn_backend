use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use serde_json::json;

use crate::utils::{
    jwt::Claims,
    app_error::AppError

};

pub fn require_roles(allowed_roles: Vec<&'static str>,)
    -> impl Fn(Request, Next)
        -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, AppError>> + Send>>
    + Clone{
        move |req: Request, next: Next| {
            let allowed_roles = allowed_roles.clone();

            Box::pin(async move{
                let claims = req.extensions().get::<Claims>();

                if let Some(claims) = claims{
                    if allowed_roles.contains(&claims.role.as_str()){
                        return Ok(next.run(req).await);
                    }
                }

                Err(AppError{
                    status: StatusCode::FORBIDDEN,
                    body: json!({
                        "success": false,
                        "messsage": "Access Denied"
                    })
                })
            })
        }
    }