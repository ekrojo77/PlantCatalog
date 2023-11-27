use std::net::SocketAddr;

use crate::config::Auth0Config;

mod models;
mod handlers;
mod utils;
mod api;
mod common;
pub mod config;

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    let auth0_config = Auth0Config::new();

    let app = api::routes::api_routes(auth0_config);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}