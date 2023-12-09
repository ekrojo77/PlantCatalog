use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable)]
#[derive(Clone)]
pub struct User {
    pub auth0_id: Option<String>,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}