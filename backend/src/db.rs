use mongodb::{Client, options::ClientOptions};
use std::env;

pub async fn connect_to_database() -> Result<Client, mongodb::error::Error> {

    let db_uri = env::var("MONGO_URI").expect("There is no MongoDB URI in .env");

    // This parses the uri and validates it
    let client_options = ClientOptions::parse(&db_uri).await?;

    // This set up connection pools internally but does not create any netwrok connections either.
    // Network connections are made when you perform any opeartion. This is to enhance performance.
    let client = Client::with_options(client_options);

    client
}