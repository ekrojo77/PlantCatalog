use axum::{extract, Json};
use crate::models::User;
use crate::edgedb_conn::create_pool;

pub async fn register_user(extract::Json(user): extract::Json<User>) -> Result<Json<User>, String> {
    let pool = create_pool().await;
    let mut conn = pool.acquire().await.unwrap();

    let query = format!(
        "INSERT User {{ username := '{}', password := '{}' }};",
        user.username,
        user.password
    );

    match conn.execute(&query).await {
        Ok(_) => Ok(Json(user)),
        Err(err) => Err(format!("Failed to insert user: {}", err)),
    }
}