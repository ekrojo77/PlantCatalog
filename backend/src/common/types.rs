use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

// This struct might also be moved to a common location if used by multiple handlers
#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}