use axum::{
    routing::{get, post},
    Router,
};

use super::handlers::{
    add_plant_handler, create_user_handler, get_user_by_username_handler, login_handler,
    refresh_token_handler, validate_token_handler,
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
        .route("/refresh_token", post(refresh_token_handler))
        .route("/add_plant", post(add_plant_handler))
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
