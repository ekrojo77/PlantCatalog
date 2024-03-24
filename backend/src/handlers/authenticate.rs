use jsonwebtoken::{encode, EncodingKey, Header};
use crate::common::types::Claims;
use crate::utils::auth::verify_password;

use super::get_users::find_user_by_username_login;
use chrono::Utc;


pub async fn login(username: &str, password: &str) -> Result<String, String> {
    let user = find_user_by_username_login(username).await.map_err(|_e| "User not found".to_string())?;

    if !verify_password(password, &user.password).map_err(|_e| "Password verification failed".to_string())? {
        return Err("Password verification failed".to_string());
    }
    
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.username,
        exp: expiration,
    };

    // Encode the JWT
    let secret_key = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
        .map_err(|_e| "JWT creation failed".to_string())
}