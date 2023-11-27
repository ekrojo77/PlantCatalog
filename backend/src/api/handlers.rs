use axum::Extension;
use axum::response::IntoResponse;
use axum::{Json, http::StatusCode};
use axum::extract::Path;
use serde_json::json;
use crate::config::Auth0Config;
use crate::models::login::LoginRequest;
use crate::{handlers::create_user::create_user, handlers::get_users::find_user_by_username,  handlers::authenticate::authenticate_user , common::types::{UserResponse, CreateUserRequest}}; 


pub async fn create_user_handler(Json(payload): Json<CreateUserRequest>) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let result = create_user(payload).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    Ok(Json(UserResponse {
        name: result.name,
        username: result.username,
        email: result.email,
    }))
}
pub async fn get_user_by_username_handler(Path(username): Path<String>) -> Result<Json<UserResponse>, (StatusCode, String)> {
    match find_user_by_username(&username).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
} 

pub async fn login_user_handler(
    Extension(auth0_config): Extension<Auth0Config>,
    Json(payload): Json<LoginRequest>,
    // Include other necessary extensions, like a database connection or Auth0 configuration if needed
) -> impl IntoResponse {
    // Here you would call your authentication logic.
    // This function could interact with Auth0 and return a JWT or an error.
    match authenticate_user(&auth0_config, &payload.username, &payload.password).await {
        Ok(token) => (
            StatusCode::OK,
            Json(json!({ "token": token }))
        ),
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": e.to_string() }))
        ),
    }
}