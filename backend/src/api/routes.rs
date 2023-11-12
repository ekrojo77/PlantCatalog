use axum::{routing::post, Router};
use super::handlers::create_user_handler;


pub fn api_routes() -> Router {
    Router::new()
        .route("/create_user", post(create_user_handler))
}