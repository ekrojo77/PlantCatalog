use std::env;

pub struct AppConfig {
    pub auth0_domain: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub management_api_token_url: String,
    pub management_api_audience: String,
    pub management_client_id: String,
    pub management_client_secret: String,
    pub connection: String,
}

impl AppConfig {
    pub fn new() -> Result<Self, env::VarError> {
        Ok(AppConfig {
            auth0_domain: env::var("AUTH0_DOMAIN")?,
            client_id: env::var("AUTH0_CLIENT_ID")?,
            client_secret: env::var("AUTH0_CLIENT_SECRET")?,
            redirect_uri: env::var("AUTH0_REDIRECT_URI")?,
            management_api_token_url: env::var("AUTH0_MANAGEMENT_API_TOKEN_URL")?,
            management_api_audience: env::var("AUTH0_MANAGEMENT_API_AUDIENCE")?,
            management_client_id: env::var("AUTH0_MANAGEMENT_CLIENT_ID")?,
            management_client_secret: env::var("AUTH0_MANAGEMENT_CLIENT_SECRET")?,
            connection: env::var("AUTH0_CONNECTION")?,
        })
    }
}