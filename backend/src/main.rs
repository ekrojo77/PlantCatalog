use axum::{handler::post, Router};
mod models;
mod handlers;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/register", post(handlers::user::register_user));

    let addr = "127.0.0.1:3000".parse().unwrap();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}