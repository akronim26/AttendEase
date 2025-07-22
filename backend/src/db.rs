use mongodb::{Client, options::ClientOptions};
use std::env;

/// Establishes a connection to the MongoDB database and returns a thread-safe client.
///
/// This function reads the MongoDB connection URI from the `MONGO_URI` environment
/// variable. It then parses the URI and creates a `mongodb::Client`.
///
/// Note that the client manages a connection pool internally. According to the
/// `mongodb` crate's documentation, the client is designed to be created once
/// and shared across threads. It will not establish a network connection until
/// an actual database operation is performed, which is a performance optimization.
pub async fn connect_to_database() -> Result<Client, mongodb::error::Error> {
    let db_uri = env::var("MONGO_URI").expect("There is no MongoDB URI in .env");

    // Parse the connection string and validate it.
    let client_options = ClientOptions::parse(&db_uri).await?;

    // Create the client with the parsed options.
    // The client will not connect to the database until the first operation is performed.
    let client = Client::with_options(client_options)?;

    Ok(client)
}
