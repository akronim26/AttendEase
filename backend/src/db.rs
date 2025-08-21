//! This module handles the database connection.

use mongodb::{Client, options::ClientOptions};
use std::env;

/// This function reads the MongoDB URI from the `MONGO_URI` environment variable,
/// parses it, and creates a new MongoDB client.
///
/// # Returns
///
/// A `Result` containing the MongoDB `Client` on success, or a `mongodb::error::Error` on failure.
pub async fn connect_to_database() -> Result<Client, mongodb::error::Error> {
    let db_uri = env::var("MONGO_URI").expect("There is no MongoDB URI in .env");

    // Parse the MongoDB URI to obtain client options.
    let client_options = ClientOptions::parse(&db_uri).await?;

    // Connects lazily, actual connection happens on first DB use.
    let client = Client::with_options(client_options)?;

    Ok(client)
}