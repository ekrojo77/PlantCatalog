use crate::models::users::{User, UserResponse};
use crate::services::user_service::{create_user, get_user_by_username};
use axum::{extract::Path, http::StatusCode, Json};

pub async fn create_user_handler(
    Json(payload): Json<User>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let result = create_user(payload)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(UserResponse {
        name: result.name,
        username: result.username,
    }))
}

pub async fn get_user_by_username_handler(
    Path(username): Path<String>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    match get_user_by_username(&username).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
