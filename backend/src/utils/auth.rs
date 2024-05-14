use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::common::types::Claims;

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

pub fn generate_jwt_token(username: &str) -> String {
    let expiration = Utc::now() + Duration::minutes(15); // JWT expires in 15 minutes
    let claims = Claims {
        sub: username.to_string(),
        exp: expiration.timestamp() as usize,
    };
    let secret_key = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .map_err(|e| e.to_string())
    .unwrap() // Convert Result to String
}
