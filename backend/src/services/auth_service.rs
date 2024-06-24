use crate::repositories::user_repository::UserRepository;
use crate::utils::auth::{generate_jwt_token, verify_password};
use std::error::Error;

pub async fn login(username: &str, password: &str) -> Result<String, String> {
    let repository = UserRepository::new();
    let user = repository
        .find_user_by_username(username)
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
    let validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    match jsonwebtoken::decode::<crate::models::login::Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(secret.as_ref()),
        &validation,
    ) {
        Ok(token_data) => Ok(token_data.claims.sub),
        Err(err) => Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Token error: {}", err),
        ))),
    }
}
