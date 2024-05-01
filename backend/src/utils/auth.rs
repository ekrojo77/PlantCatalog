use std::{collections::HashMap, error::Error, io};

use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{common::types::Claims, models::login::RefreshToken};

use super::db_utils::create_db_client;
//use edgedb_protocol::model::DecodeScalar;

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    verify(password, hash)
}

/* pub async fn validate_refresh_token(refresh_token: &str) -> bool {
    let token_record = get_refresh_token(refresh_token).await.unwrap();
    
    if token_record.is_none() || token_record.unwrap().is_revoked || token_record.unwrap().expires_at < Utc::now() {
        return Err("Invalid or expired refresh token".to_string());
    }

    Ok(true)
}

pub async fn get_refresh_token(token: &str) -> Result<RefreshToken, Box<dyn Error>> {
    let client = create_db_client().await.unwrap();

    let mut params = HashMap::new();
    params.insert("refresh_tokem", token);
    
    let query = r#"
        SELECT RefreshToken {
            token,
            user,
            expires_at,
            is_revoked
        }
        FILTER .token = <str>$0
    "#;

    // Execute the query
    let user_option: Option<RefreshToken> = client.query_single(
        query,
        &(token,)
    ).await?;

    Ok(user_option.unwrap())
}

pub async fn add_refresh_token_to_db(username: &str) -> Result<RefreshToken, edgedb_tokio::Error> {
    let client = create_db_client().await.unwrap();

    let query = r###"
        INSERT RefreshToken {
            token := <str>$1,
            user := <User>$2,
            expires_at := <datetime>$3,
            is_revoked := <bool>$4
        }
    "###;

    client.execute(query, &(
        &token.token, &token.username, &token.expires_at, &token.is_revoked
    )).await?;

    Ok(token.clone())
} */

pub fn generate_jwt_token(username: &str) -> String {
    let expiration = Utc::now() + Duration::minutes(15); // JWT expires in 15 minutes
    let claims = Claims {
        sub: username.to_string(),
        exp: expiration.timestamp() as usize,
    };
    let secret_key = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret_key.as_ref()))
        .map_err(|e| e.to_string())
        .unwrap() // Convert Result to String
}