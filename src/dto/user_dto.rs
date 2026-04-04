use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest{
    pub name: String,
    pub email: String,
    pub password: String,
    pub role: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}