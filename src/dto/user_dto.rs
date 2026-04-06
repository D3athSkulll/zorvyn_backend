use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateUserRequest{
    #[validate(length(min = 2, message = "Name too short"))]
    pub name: String,

    #[validate(email(message = "Invalid email"))]
    pub email: String,
    
    #[validate(length(min = 6, message = "Password must be at least 6 chars"))]
    pub password: String,
    
    #[validate(custom(function = "validate_role"))]
    pub role: String,
}


#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// custom validator
fn validate_role(role: &str) -> Result<(), validator::ValidationError> {
    match role {
        "admin" | "analyst" | "viewer" => Ok(()),
        _ => Err(validator::ValidationError::new("invalid_role")),
    }
}