use axum::extract::Path;
use axum::{http::StatusCode, Json};
use serde::Deserialize;

use crate::common::types::UserResponse;
use crate::handlers::authenticate::{login, validate_token};
use crate::models::login::{LoginRequest, LoginResponse};
use crate::models::plants::Plant;
use crate::models::users::User;

use crate::utils::auth::generate_jwt_token;
use crate::{handlers::create_user::create_user, handlers::get_users::find_user_by_username};

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
    match find_user_by_username(&username).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn login_handler(
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let token = login(&payload.username, &payload.password)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    Ok(Json(LoginResponse { token }))
}
#[derive(Deserialize)]
pub struct TokenRequest {
    pub token: String,
}

pub async fn validate_token_handler(
    token: Json<TokenRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    match validate_token(token.0.token).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err((StatusCode::UNAUTHORIZED, e.to_string())),
    }
}

pub async fn refresh_token_handler(
    Json(payload): Json<TokenRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let refresh_token = payload.token;
    match validate_token(refresh_token).await {
        Ok(username) => {
            let new_access_token = generate_jwt_token(&username);
            Ok(Json(LoginResponse {
                token: new_access_token,
            }))
        }
        Err(e) => Err((StatusCode::UNAUTHORIZED, e.to_string())),
    }
}

pub async fn add_plant_handler(
    Json(payload): Json<Plant>,
) -> Result<Json<Plant>, (StatusCode, String)> {
    let result = add_plant(payload)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e))?;

    Ok(Json(result))
}
