use std::net::SocketAddr;

mod models;
mod handlers;
mod utils;
mod api;
mod common;

#[tokio::main]
async fn main() {
    let app = api::routes::api_routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}