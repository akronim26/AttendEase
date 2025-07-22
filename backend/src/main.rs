mod db;
mod state;

// The socket addr is used for creating a combination of IP and port
// The axum is used for routing where the extract is used for injecting the app state into the route
use crate::state::AppState;
use axum::{Router, extract::Extension, routing::get};
use dotenvy::dotenv;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().expect(".env does not exist");

    let client = db::connect_to_database().await?;
    let shared_client = Arc::new(client);

    let app_state = AppState {
        db_client: shared_client.clone(),
    };

    let app = Router::new()
        .route("/", get(root_handler))
        .layer(Extension(app_state)); // inject the shared state

    let address = SocketAddr::from(([127, 0, 0, 1], 3000)); // explicitly defining the port and IP address
    let listener = TcpListener::bind(address).await?;
    println!("listening on {}", address);
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root_handler(Extension(state): Extension<AppState>) -> &'static str {
    // Accessing the db_client to show the compiler it's being used.
    let _ = &state.db_client;
    "Attendance portal backend is running"
}
