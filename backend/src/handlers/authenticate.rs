use std::sync::Arc;

use oauth2::{CsrfToken, basic::BasicClient, Scope};

use crate::common::types::OAuthConfig;

pub async fn construct_oauth_redirect(oauth_config: Arc<OAuthConfig>) -> Result<String, String> {
    let client = BasicClient::new(
        oauth_config.client_id.clone(),
        Some(oauth_config.client_secret.clone()),
        oauth_config.auth_url.clone(),
        Some(oauth_config.token_url.clone())
    )
    .set_redirect_uri(oauth_config.redirect_url.clone());

    // Generate the authorization URL
    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        // .set_pkce_challenge(pkce_challenge) // Add this if you're using PKCE
        .url();

    Ok(authorize_url.to_string())
}