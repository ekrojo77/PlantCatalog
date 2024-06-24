use crate::models::login::{LoginRequest, LoginResponse, TokenRequest};
use crate::services::auth_service::{login, validate_token};
use axum::{http::StatusCode, Json};

pub async fn login_handler(
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let token = login(&payload.username, &payload.password)
        .await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e))?;

    Ok(Json(LoginResponse { token }))
}

pub async fn validate_token_handler(
    Json(payload): Json<TokenRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    match validate_token(payload.token).await {
        Ok(_) => Ok(StatusCode::OK),
        Err(e) => Err((StatusCode::UNAUTHORIZED, e.to_string())),
    }
}
