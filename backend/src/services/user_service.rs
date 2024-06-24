use crate::models::api::user::{User, UserResponse};
use crate::repositories::user_repository::UserRepository;
use std::error::Error;

pub async fn create_user(request: User) -> Result<UserResponse, String> {
    let repository = UserRepository::new();
    let password_hash = crate::utils::auth::hash_password(request.password.as_str())
        .map_err(|_| "Failed to hash password".to_string())?;

    let new_user = User {
        name: request.name,
        username: request.username,
        password: password_hash,
        id: todo!(),
        created_at: todo!(),
    };
    let created_user = repository
        .add_user(&new_user)
        .await
        .map_err(|e| format!("Failed to add user to database: {}", e))?;

    Ok(UserResponse {
        name: created_user.name,
        username: created_user.username,
    })
}

pub async fn get_user_by_username(username: &str) -> Result<UserResponse, Box<dyn Error>> {
    let repository = UserRepository::new();
    repository.find_user_by_username(username).await
}
