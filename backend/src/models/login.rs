use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
 #[derive(Serialize, Deserialize )]
pub struct Auth0Response {
    pub access_token: String,
}