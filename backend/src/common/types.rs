use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

// This struct might also be moved to a common location if used by multiple handlers
#[derive(Serialize)]
pub struct UserResponse {
    pub name: String,
    pub username: String,
    pub email: String,
}