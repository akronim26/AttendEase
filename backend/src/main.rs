mod db; 
mod state;

// The socket addr is used for creating a combination of IP and port
// The axum is used for routing where the extract is used for injecting the app state into the route
use std::{net::SocketAddr, sync::Arc};
use dotenvy::dotenv;
use axum::{Router, routing::get, extract::Extension};
use crate::state::AppState;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env does not exist");

    let client = db.connect_to_database().await?;
    let shared_client = Arc::new(client);

    let app_state = AppState {
        db_client: shared_client.clone();
    }

    let app = Router::new().
    route("/", get(root_handler))
    .layer(Extension(app_state)); // inject the shared state


    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(address)
    .serve(app.into_make_service())
    .await?;

    Ok(())

}

fn root_handler() -> &'static str {
    "Attendance portal backend is running"
}