use edgedb::credentials::Credentials;
use edgedb::Builder;
use std::time::Duration;

pub async fn create_pool() -> edgedb::Pool {
    let creds = Credentials::from_path_or_default().await.unwrap();
    let builder = Builder::from_credentials(&creds)
        .wait_timeout(Duration::from_secs(10))
        .max_size(10); // Adjust as per your needs

    builder.get_pool().await.unwrap()
}