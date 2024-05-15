use edgedb_derive::Queryable;
use serde::{Deserialize, Serialize};

use super::plants::Plant;

#[derive(Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct User {
    pub name: String,
    pub username: String,
    pub password: String,
    pub plants: Option<Vec<Plant>>,
}
