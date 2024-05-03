use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::users::User;

#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RefreshToken{
    token: String,
    user: User,
    expires_at: DateTime<Utc>,
    is_revoked: bool,
}