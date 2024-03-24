use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[derive(Clone)]
pub struct User{
    pub name: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
}