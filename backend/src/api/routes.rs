use std::sync::Arc;

use axum::{routing::{post, get}, Router, Extension};

use crate::common::types::OAuthConfig;

use super::handlers::{create_user_handler, get_user_by_username_handler, login_handler};


pub fn api_routes(oauth_config: Arc<OAuthConfig>) -> Router {

    Router::new()
        .route("/create_user", post(create_user_handler))
        .route("/get_user_by_username/:username", get(get_user_by_username_handler))
        .route("/login", get(login_handler))
        .layer(Extension(oauth_config.clone()))
}