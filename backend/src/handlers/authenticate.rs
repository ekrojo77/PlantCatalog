use std::fmt;

use reqwest::{Client, Error as ReqwestError};
use serde_json::json;

use crate::{config::Auth0Config, models::login::Auth0Response};

// Error handling
#[derive(Debug)]
pub enum AuthError {
    RequestError(ReqwestError),
    InvalidCredentials,
    // You can add more specific error types as needed
}

impl From<ReqwestError> for AuthError {
    fn from(err: ReqwestError) -> Self {
        AuthError::RequestError(err)
    }
}

impl std::error::Error for AuthError {}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::RequestError(e) => write!(f, "Request error: {}", e),
            AuthError::InvalidCredentials => write!(f, "Invalid credentials"),
            // Handle other variants as needed
        }
    }
}

// Function to authenticate the user
pub async fn authenticate_user(config: &Auth0Config, username: &str, password: &str) -> Result<String, AuthError> {
    let client = Client::new();

    // Structure the request payload as required by Auth0
    let params = json!({
        "client_id": config.client_id,
        "client_secret": config.client_secret,
        "audience": "...", // Set your Auth0 audience
        "grant_type": "password",
        "username": username,
        "password": password,
        "scope": "openid profile email", // Adjust the scope as needed
    });

    let res = client
        .post(format!("https://{}/oauth/token", config.domain))
        .json(&params)
        .send()
        .await?;

    // Check if the request was successful
    if res.status().is_success() {
        let auth0_response: Auth0Response = res.json().await?;
        Ok(auth0_response.access_token)
    } else {
        // Handle different kinds of errors here
        Err(AuthError::InvalidCredentials)
    }
}