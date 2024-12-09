use mongodb::{options::ClientOptions, Client, Database};
use tracing::log::info;

// test push
pub async fn dbconnect() -> mongodb::error::Result<Database> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://root:123456@localhost:27017")
        .await
        .unwrap();

    // Get a handle to the deployment.
    client_options.app_name = Some("My App".to_string());

    // List the names of the databases in that deployment.
    let client = Client::with_options(client_options)?;

    let db = client.database("mydb");

    info!("Connected to database");

    Ok(db)
}
