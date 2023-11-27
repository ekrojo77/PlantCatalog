use serde::Deserialize;
use std::env;

#[derive(Deserialize, Clone)]
pub struct Auth0Config {
    pub domain: String,
    pub client_id: String,
    pub client_secret: String,
}

impl Auth0Config {
    pub fn new() -> Self {
        Self {
            domain: env::var("AUTH0_DOMAIN").expect("AUTH0_DOMAIN must be set"),
            client_id: env::var("AUTH0_CLIENT_ID").expect("AUTH0_CLIENT_ID must be set"),
            client_secret: env::var("AUTH0_CLIENT_SECRET").expect("AUTH0_CLIENT_SECRET must be set"),
        }
    }
}