use crate::utils::auth::hash_password;
use crate::utils::db_utils::create_db_client;
use crate::{common::types::UserResponse, models::users::User};

pub async fn create_user(request: User) -> Result<UserResponse, String> {
    let _client = create_db_client()
        .await
        .map_err(|_| ("Failed to get database client".to_string()))?;

    let password_hash = hash_password(request.password.as_str())
        .map_err(|_| "Failed to hash password".to_string())?;

    print!("Password hash: {}", password_hash);

    let new_user = User {
        name: request.name,
        username: request.username,
        password: password_hash,
        plants: None,
    };
    let created_user = add_user_to_db(&new_user)
        .await
        .map_err(|e| format!("Failed to add user to database: {}", e))?;

    Ok(UserResponse {
        name: created_user.name,
        username: created_user.username,
    })
}
pub async fn add_user_to_db(user: &User) -> Result<User, edgedb_tokio::Error> {
    println!("Adding user to db: {:?}", user);
    let client = create_db_client().await.unwrap();

    let query = r###"
        INSERT User {
            name := <str>$0,
            username := <str>$1,
            password := <str>$2
        }
    "###;
    println!("Query: {}", user.name);
    println!("Query: {}", user.username);
    println!("Query: {}", user.password);
    client
        .execute(query, &(&user.name, &user.username, &user.password))
        .await?;

    Ok(user.clone())
}
