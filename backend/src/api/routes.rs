use axum::{routing::{get, post}, Router };

use super::handlers::{create_user_handler, get_user_by_username_handler, login_handler};
use tower_http::cors::{Any, CorsLayer};
use http::Method;



pub fn api_routes() -> Router {

   /*  let cors = CorsLayer::new()
    .allow_methods(vec![Method::GET, Method::POST, Method::DELETE, Method::PUT, Method::PATCH])
    .allow_origin(tower_http::cors::Any )
    .allow_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
    .allow_credentials(false); */
    Router::new()
        .route("/create_user", post(create_user_handler))
        .route("/get_user_by_username/:username", get(get_user_by_username_handler))
        .route("/login", post(login_handler))
        .layer(CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([Method::GET, Method::POST, Method::DELETE, Method::PUT, Method::PATCH])
            .allow_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION])
        )
        //.route("/refresh_token", post(refresh_token_handler))
}