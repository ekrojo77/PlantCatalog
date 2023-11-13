use crate::{models::users::User, utils::hash_password::hash_password, common::types::{UserResponse, CreateUserRequest}};
use crate::utils::db_utils::create_db_client;

pub async fn create_user(request: CreateUserRequest) -> Result<UserResponse, String> {
    let _client = create_db_client().await
        .map_err(|_| ("Failed to get database client".to_string()))?;

    let hashed_password = hash_password(&request.password)
        .map_err(|_| ("Failed to hash password".to_string()))?;

    let new_user = User {
        name: request.name,
        username: request.username,
        email: request.email,
        password_hash: hashed_password,
    };
    let created_user = add_user_to_db(&new_user).await
        .map_err(|e| format!("Failed to add user to database: {}", e))?;

    Ok(UserResponse {
        name: created_user.name,
        username: created_user.username,
        email: created_user.email,
    })
}
pub async fn add_user_to_db(user: &User) -> Result<User, edgedb_tokio::Error> {
    let client = create_db_client().await.unwrap();
    let query = r###"
        INSERT User {
            name := <str>$0,
            username := <str>$1,
            email := <str>$2,
            password_hash := <str>$3
        }
    "###;

    client.execute(query, &(
        &user.name,
        &user.username,
        &user.email,
        &user.password_hash,
    )).await?;

    Ok(user.clone())
}