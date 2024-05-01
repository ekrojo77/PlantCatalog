use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub username: String,
    pub password: String,
}

// This struct might also be moved to a common location if used by multiple handlers
#[derive(Serialize)]
pub struct UserResponse {
    pub name: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct UserPasswordResponse {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
    pub username: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    pub token: String,
}
