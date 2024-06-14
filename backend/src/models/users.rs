use edgedb_protocol::queryable;
use edgedb_tokio::Queryable;
use serde::{Deserialize, Serialize};

use super::plants::Plant;

#[derive(Debug, Serialize, Queryable, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub username: String,
    pub password: String,
    #[queryable]
    pub plants: Option<Vec<Plant>>,
}
