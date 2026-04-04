use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use crate::utils::jwt::Claims;

pub fn require_role(required_role: &'static str)
    -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, StatusCode>> + Send>>
    + Clone{
        move |req: Request, next: Next| {
            let required_role = required_role.to_string();

            Box::pin(async move{
                let claims = req.extensions().get::<Claims>();

                if let Some(claims) = claims{
                    if claims.role == required_role{
                        return Ok(next.run(req).await);
                    }
                }

                Err(StatusCode::FORBIDDEN)
            })
        }
    }