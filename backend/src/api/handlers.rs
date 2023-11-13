use axum::{Json, http::StatusCode};
use axum::extract::Path;
use crate::{handlers::create_user::create_user, handlers::get_users::find_user_by_username, common::types::{UserResponse, CreateUserRequest}}; 


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