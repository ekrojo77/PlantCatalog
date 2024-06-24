use crate::models::api::user::UserResponse;
use crate::{models::api::user::User, utils::db_utils::create_db_client};
use std::error::Error;

pub struct UserRepository;

impl UserRepository {
    pub fn new() -> Self {
        Self
    }

    pub async fn add_user(&self, user: &User) -> Result<User, edgedb_tokio::Error> {
        let client = create_db_client().await.unwrap();

        let query = r###"
            INSERT User {
                name := <str>$0,
                username := <str>$1,
                password := <str>$2
            }
        "###;
        client
            .execute(query, &(&user.name, &user.username, &user.password))
            .await?;

        Ok(user)
    }

    pub async fn find_user_by_username(
        &self,
        username: &str,
    ) -> Result<UserResponse, Box<dyn Error>> {
        let client = create_db_client().await.unwrap();

        let query = r#"
            SELECT User {
                name,
                username,
            }
            FILTER .username = <str>$0
        "#;

        match client.query_single::<User, _>(query, &(username,)).await {
            Ok(Some(user)) => Ok(UserResponse {
                username: user.username,
                name: user.name,
            }),
            Ok(None) => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "User not found",
            ))),
            Err(e) => Err(Box::new(e)),
        }
    }
}
