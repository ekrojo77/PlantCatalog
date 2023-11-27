use std::time::{SystemTime, UNIX_EPOCH};

use axum::{http::Request, http::StatusCode, middleware::Next, response::{IntoResponse, Response}};
use jsonwebtoken::{encode, EncodingKey, Header, DecodingKey, Validation, decode};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_jwt(user_id: &str) -> String {
    let expiration = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .expect("Time went backwards")
    .as_secs() + 3600;
    let claims = Claims { sub: user_id.to_owned(), exp: expiration as usize };
    encode(&Header::default(), &claims, &EncodingKey::from_secret("your-secret".as_ref())).unwrap()
}

async fn validate_token<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    if let Some(token_result) = extract_token(&req).await {
        match token_result {
            Ok(token) => {
                // Validate token here
                let validation = Validation::default();
                // Use the key and algorithm you used to sign the JWT
                match decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &validation) {
                    Ok(_) => return next.run(req).await,
                    Err(_) => return Response::builder().status(StatusCode::UNAUTHORIZED).body("Invalid token".into()).unwrap(),
                }
            }
            Err(_) => return Response::builder().status(StatusCode::UNAUTHORIZED).body("Token required".into()).unwrap(),
        }
    } else {
        return Response::builder().status(StatusCode::UNAUTHORIZED).body("Token required".into()).unwrap();
    }
}

async fn extract_token<B>(req: &Request<B>) -> Option<Result<String, ()>> {
    if let Ok(TypedHeader(Authorization(bearer))) = TypedHeader::<Authorization<headers::authorization::Bearer>>::from_request(req).await {
        Some(Ok(bearer.token().to_string()))
    } else {
        None
    }
}