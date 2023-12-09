use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, RedirectUrl};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub username: String,
    pub email: String,
}

// This struct might also be moved to a common location if used by multiple handlers
#[derive(Serialize)]
pub struct UserResponse {
    pub name: String,
    pub username: String,
    pub email: String,
}

pub struct OAuthConfig {
    pub client_id: ClientId,
    pub client_secret: ClientSecret,
    pub auth_url: AuthUrl,
    pub token_url: TokenUrl,
    pub redirect_url: RedirectUrl,
    pub audience: String,
    pub connection: String
}

#[derive(Debug, Serialize)]
pub struct Auth0CreateUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub connection: String,
    // Add other fields as needed
}

#[derive(Debug,Deserialize)]
pub struct Auth0CreateUserResponse {
    pub user_id: Option<String>,
    // Add other fields as needed
}