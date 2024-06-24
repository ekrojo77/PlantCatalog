use axum::{
    routing::{get, post},
    Router,
};

use super::controllers::{
    auth_controller::{login_handler, validate_token_handler},
    user_controller::{create_user_handler, get_user_by_username_handler},
};

use http::Method;
use tower_http::cors::{Any, CorsLayer};

pub fn api_routes() -> Router {
    Router::new()
        .route("/create_user", post(create_user_handler))
        .route(
            "/get_user_by_username/:username",
            get(get_user_by_username_handler),
        )
        .route("/login", post(login_handler))
        .route("/validate_token", post(validate_token_handler))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::DELETE,
                    Method::PUT,
                    Method::PATCH,
                ])
                .allow_headers(vec![
                    http::header::CONTENT_TYPE,
                    http::header::AUTHORIZATION,
                ]),
        )
}
