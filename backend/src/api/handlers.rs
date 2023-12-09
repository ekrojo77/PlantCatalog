use std::sync::Arc;



use axum::response::{IntoResponse, Redirect};
use axum::{Json, http::StatusCode, Extension};
use axum::extract::Path;


use crate::common::types::OAuthConfig;
use crate::models::users::User;
use crate::{handlers::create_user::create_user, handlers::get_users::find_user_by_username, handlers::authenticate::construct_oauth_redirect, common::types::UserResponse}; 

pub async fn create_user_handler(
    Extension(config): Extension<Arc<OAuthConfig>>, 
    Json(payload): Json<User>
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let result = create_user(config, payload).await
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

pub async fn login_handler(Extension(config): Extension<Arc<OAuthConfig>>, ) -> Result<impl IntoResponse, (StatusCode, String)> {
    match construct_oauth_redirect(config).await {
        Ok(redirect_url) => Ok(Redirect::temporary(redirect_url.as_str()).into_response()),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e)),
    }
}