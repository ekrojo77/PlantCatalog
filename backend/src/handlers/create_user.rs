use std::sync::Arc;

use reqwest::Client;
use serde_json::json;

use crate::{models::users::User, common::types::{UserResponse, Auth0CreateUserResponse, Auth0CreateUserRequest, OAuthConfig}};
use crate::utils::db_utils::create_db_client;

pub async fn create_user(config: Arc<OAuthConfig>, request: User) -> Result<UserResponse, String> {

    let auth0_user = create_auth0_user(config,&request).await
        .map_err(|e| format!("Failed to create user in Auth0: {}", e))?;

        let auth0_id = match auth0_user.user_id {
            Some(id) => id,
            None => return Err("Failed to obtain auth0_id from Auth0".to_string()),
        };

    let _client = create_db_client().await
        .map_err(|_| ("Failed to get database client".to_string()))?;

    let new_user = User {
        auth0_id: Some(auth0_id),
        name: request.name,
        username: request.username,
        email: request.email,
        password: request.password
    };
    let created_user = add_user_to_db(&new_user).await
        .map_err(|e| format!("Failed to add user to database: {}", e))?;

    Ok(UserResponse {
        name: created_user.name,
        username: created_user.username,
        email: created_user.email,
    })
}
pub async fn add_user_to_db(user: &User) -> Result<User, edgedb_tokio::Error> {
    let client = create_db_client().await.unwrap();

    let auth0_id = user.auth0_id.as_ref().expect("auth0_id is not set");

    let query = r###"
        INSERT User {
            auth0_user_id := <str>$0,
            name := <str>$1,
        }
    "###;

    client.execute(query, &(
        auth0_id,
        &user.name,
    )).await?;

    Ok(user.clone())
}

async fn create_auth0_user(oauth_config: Arc<OAuthConfig>, request: &User) -> Result<Auth0CreateUserResponse, String> {
    let api_token = get_auth0_management_api_token(oauth_config.clone()).await
        .map_err(|e| format!("Failed to get Auth0 API token: {}", e))?;

    let client = Client::new();
    let user_request = Auth0CreateUserRequest {
        email: request.email.clone(),
        username: request.username.clone(),
        password: request.password.clone(),
        connection: oauth_config.connection.clone()
    };

    let url = format!("{}api/v2/users", oauth_config.auth_url.to_string());

    let res = client.post(&url)
        .bearer_auth(api_token)
        .json(&user_request)
        .send()
        .await
        .map_err(|e| format!("Auth0 request failed: {}", e))?;

    if res.status().is_success() {
        res.json::<Auth0CreateUserResponse>().await
            .map_err(|e| format!("Failed to parse Auth0 response: {}", e))
    } else {
        Err(format!("Auth0 user creation failed with status: {}", res.status()))
    }
}

async fn get_auth0_management_api_token(oauth_config: Arc<OAuthConfig>) -> Result<String, String> {
    let client = Client::new();
    let token_url  = oauth_config.token_url.to_string();

    let response = client.post(&token_url)
        .json(&json!({
            "client_id": oauth_config.client_id,
            "client_secret": oauth_config.client_secret,
            "audience": oauth_config.audience, // Ensure you have `audience` in OAuthConfig
            "grant_type": "client_credentials"
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if response.status().is_success() {
        let token_data: serde_json::Value = response.json().await
            .map_err(|e| e.to_string())?;
        token_data["access_token"].as_str()
            .ok_or_else(|| "Failed to extract access token".to_string())
            .map(|s| s.to_string())
    } else {
        Err(format!("Failed to obtain Auth0 management API token: {}", response.status()))
    }
}