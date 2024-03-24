use axum::{routing::{post, get}, Router, };

use super::handlers::{create_user_handler, get_user_by_username_handler, login_handler};


pub fn api_routes() -> Router {

    Router::new()
        .route("/create_user", post(create_user_handler))
        .route("/get_user_by_username/:username", get(get_user_by_username_handler))
        .route("/login", post(login_handler))
}