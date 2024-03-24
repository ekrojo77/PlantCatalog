use crate::utils::auth::hash_password;
use crate::{models::users::User, common::types::UserResponse};
use crate::utils::db_utils::create_db_client;

pub async fn create_user(request: User) -> Result<UserResponse, String> {

    let _client = create_db_client().await
        .map_err(|_| ("Failed to get database client".to_string()))?;

    let password_hash = hash_password(request.password_hash.as_str())
        .map_err(|_| "Failed to hash password".to_string())?;

    let new_user = User {  
        name: request.name,
        username: request.username,
        email: request.email,
        password_hash: password_hash
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
            name := <str>$1,
            username := <str>$2,
            email := <str>$3,
            password_hash := <str>$4
        }
    "###;

    client.execute(query, &(
        &user.name, &user.username, &user.email, &user.password_hash
    )).await?;

    Ok(user.clone())
}