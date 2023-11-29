use axum::{routing::{post, get}, Router, Extension};

use crate::config::Auth0Config;

use super::handlers::{create_user_handler, get_user_by_username_handler, login_user_handler};


pub fn api_routes(auth0_config: Auth0Config) -> Router {

    Router::new()
        .route("/create_user", post(create_user_handler))
        .route("/get_user_by_username/:username", get(get_user_by_username_handler))
        .route("/login", post(login_user_handler))
        .layer(Extension(auth0_config))
}