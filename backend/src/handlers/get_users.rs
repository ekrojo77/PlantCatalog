use std::collections::HashMap;
use std::error::Error;
use std::io;

use crate::common::types::UserResponse;
use crate::models::users::User;
use crate::utils::db_utils::create_db_client;


pub async fn find_user_by_username(username: &str) -> Result<UserResponse, Box<dyn Error>> {
    let client = create_db_client().await.unwrap();

    let mut params = HashMap::new();
    params.insert("username", username);


    let query = r#"
        SELECT User {
            name,
            username,
            email,
            password_hash
        }
        FILTER .username = <str>$0
    "#;

    // Execute the query
    let user_option: Option<User> = client.query_single(
        query,
        &(username,)
    ).await?;

    user_option.map(|user| UserResponse{
        name: user.name,
        username: user.username,
        email: user.email,
    }).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "User not found").into())
}