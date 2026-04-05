use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub struct AppError{
    pub status: StatusCode,
    pub message: String,
}

impl IntoResponse for AppError{
    fn into_response(self) -> Response {
        let body = Json(json!({
            "success": false,
            "message": self.message
        }));

        (self.status, body).into_response()
    }
}