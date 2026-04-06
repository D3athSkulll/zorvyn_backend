use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json,Value};

#[derive(Debug)]
pub struct AppError{
    pub status: StatusCode,
    pub body: Value,
}

impl IntoResponse for AppError{
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}