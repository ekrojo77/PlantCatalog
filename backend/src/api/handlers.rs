use axum::{Json, http::StatusCode};
use crate::{handlers::create_user::create_user, common::types::{UserResponse, CreateUserRequest}}; // Importing from your business logic handlers


pub async fn create_user_handler(Json(payload): Json<CreateUserRequest>) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let result = create_user(payload).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(UserResponse {
        id: result.id,
        name: result.name,
        email: result.email,
    }))
}