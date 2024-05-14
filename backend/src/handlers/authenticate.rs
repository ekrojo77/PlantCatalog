use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};

use std::error::Error;
use std::io;

use crate::{
    common::types::Claims,
    utils::auth::{generate_jwt_token, verify_password},
};

use super::get_users::find_user_by_username_login;

pub async fn login(username: &str, password: &str) -> Result<String, String> {
    let user = find_user_by_username_login(username)
        .await
        .map_err(|_e| "User not found".to_string())?;

    if !verify_password(password, &user.password)
        .map_err(|_e| "Password verification failed".to_string())?
    {
        return Err("Password verification failed".to_string());
    }

    Ok(generate_jwt_token(username))
}

pub async fn validate_token(token: String) -> Result<String, Box<dyn Error>> {
    let secret = std::env::var("JWT_SECRET")?;
    let validation = Validation::new(Algorithm::HS256);
    match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(_token_data) => Ok(_token_data.claims.sub),
        Err(err) => Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Token error: {}", err),
        ))),
    }
}
