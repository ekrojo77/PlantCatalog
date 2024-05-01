use axum::{Json, http::StatusCode};
use axum::extract::Path;


use crate::models::users::User;
use crate::models::login::{LoginRequest, LoginResponse};
use crate::common::types::UserResponse;
use crate::handlers::authenticate::login;

use crate::{handlers::create_user::create_user, handlers::get_users::find_user_by_username}; 

pub async fn create_user_handler(
    Json(payload): Json<User>
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    println!("Payload: {:?}", payload);
    let result = create_user(payload).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;
    
    Ok(Json(UserResponse {
        name: result.name,
        username: result.username,
    }))
}
pub async fn get_user_by_username_handler(Path(username): Path<String>) -> Result<Json<UserResponse>, (StatusCode, String)> {
    match find_user_by_username(&username).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
} 

pub async fn login_handler(
    Json(payload): Json<LoginRequest>
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let token = login(&payload.username, &payload.password).await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;
    
    Ok(Json(LoginResponse { token }))
}

/*pub async fn refresh_token_handler(Json(payload): Json<RefreshRequest>) -> Result<(StatusCode, Json<TokenResponse>), (StatusCode, String)> {
    // Validate the refresh token (e.g., check its expiry, verify signature, etc.)
    let is_valid = validate_refresh_token(&payload.refresh_token).await;

    if is_valid {
        // Generate a new JWT
        let new_jwt = generate_jwt_token(&payload.username.to_string());
        
        Ok((StatusCode::OK, Json(TokenResponse { token: new_jwt })))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid refresh token".to_string()))
    }
}*/