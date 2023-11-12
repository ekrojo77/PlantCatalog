use crate::{models::users::User, utils::hash_password::hash_password, common::types::{UserResponse, CreateUserRequest}};
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

    let created_user = new_user; //Implement
    
    Ok(UserResponse {
        id: created_user.id,
        name: created_user.name,
        email: created_user.email,
    })
}