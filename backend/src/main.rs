use std::net::SocketAddr; 

mod models;
mod handlers;
mod utils;
mod api;
mod common;

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    let app = api::routes::api_routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}