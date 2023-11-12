use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[derive(Clone)]
pub struct User {
    pub name: String,
    pub email: String,
    pub password_hash: String,
}