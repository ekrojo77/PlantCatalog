use edgedb_tokio::{Client};
use axum::http::StatusCode;

pub async fn create_db_client() -> Result<Client, (StatusCode, String)> {
    edgedb_tokio::create_client().await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to connect to database: {}", e)))
}