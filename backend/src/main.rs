use std::{net::SocketAddr, sync::Arc};

use oauth2::{ClientId, ClientSecret, AuthUrl, TokenUrl, RedirectUrl};

use crate::{config::AppConfig, common::types::OAuthConfig};

mod models;
mod handlers;
mod utils;
mod api;
mod common;
pub mod config;

#[tokio::main]
async fn main() {

    dotenv::dotenv().ok();

    let config= AppConfig::new().expect("Failed to load configuration");

    let oauth_config = Arc::new(OAuthConfig {
        client_id: ClientId::new(config.client_id),
        client_secret: ClientSecret::new(config.client_secret),
        auth_url: AuthUrl::new(format!("https://{}/", config.auth0_domain)).expect("Invalid auth URL"),
        token_url: TokenUrl::new(config.management_api_token_url).expect("Invalid token URL"),
        redirect_url: RedirectUrl::new(config.redirect_uri).expect("Invalid redirect URL"),
        audience: config.management_api_audience,
        connection: config.connection,
    });

    let app = api::routes::api_routes(oauth_config);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}