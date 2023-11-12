use crate::{models::users::User, utils::{hash_password::hash_password, db_utils::execute_query}, common::types::{UserResponse, CreateUserRequest}};
use uuid::Uuid;
use crate::utils::db_utils::create_db_client;

pub async fn create_user(request: CreateUserRequest) -> Result<UserResponse, String> {
    let _client = create_db_client().await
        .map_err(|_| ("Failed to get database client".to_string()))?;

    let hashed_password = hash_password(&request.password)
        .map_err(|_| ("Failed to hash password".to_string()))?;

    let new_user = User {
        id: Uuid::new_v4(),
        name: request.name,
        email: request.email,
        password_hash: hashed_password,
    };
    let created_user = add_user_to_db(&new_user).await
        .map_err(|_| ("Failed to add user to database".to_string()))?;

    Ok(UserResponse {
        id: created_user.id,
        name: created_user.name,
        email: created_user.email,
    })
}
pub async fn add_user_to_db(user: &User) -> Result<User, edgedb_tokio::Error> {
    let client = create_db_client().await
        .map_err(|_| ("Failed to add user to database".to_string()))?;
    let query = r###"
        INSERT User {
            id := <uuid>$id,
            name := <str>$name,
            email := <str>$email,
            password_hash := <str>$password_hash
        }
    "###;

    client.execute(query, &(
        ("id", user.id),
        ("name", &user.name),
        ("email", &user.email),
        ("password_hash", &user.password_hash),
    )).await?;

    Ok(user.clone())
}