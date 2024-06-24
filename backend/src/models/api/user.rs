use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime; // Add this line to import the `UserRepository` module or crate

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub username: String,
    pub password: String,
    pub created_at: OffsetDateTime,
}

impl User {
    pub fn from(user: UserRepository) -> Self {
        // Update the function signature to use `UserRepository` directly
        let created_at: OffsetDateTime = user.created_at.assume_utc();
        Self {
            id: user.id,
            name: user.name,
            username: user.username,
            password: user.password,
            created_at,
        }
    }
}

#[derive(Serialize)]
pub struct UserResponse {
    pub name: String,
    pub username: String,
}
