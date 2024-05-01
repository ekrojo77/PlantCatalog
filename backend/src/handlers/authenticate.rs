use crate::utils::auth::{generate_jwt_token, verify_password};

use super::get_users::find_user_by_username_login;


pub async fn login(username: &str, password: &str) -> Result<String, String> {
    let user = find_user_by_username_login(username).await.map_err(|_e| "User not found".to_string())?;

    if !verify_password(password, &user.password).map_err(|_e| "Password verification failed".to_string())? {
        return Err("Password verification failed".to_string());
    }
    
    Ok(generate_jwt_token(username))
}