//! This is the main entry point of the application.

mod db;
mod state;
mod routes {
    pub mod student_route;
}
mod models {
    pub mod student_model;
}
mod error;

use axum::{Extension, Router, routing::{get, post}};
use dotenvy::dotenv;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;
use crate::routes::student_route::add_student;
use crate::state::AppState;

/// The main function of the application.
///
/// This function initializes the application by loading the environment variables,
/// connecting to the database, creating the application state, and starting the
/// HTTP server.
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
        .route("/students", post(add_student))
        .layer(Extension(app_state)); // Injects the application state into all routes.

    let address = SocketAddr::from(([127, 0, 0, 1], 3000)); // Defines the IP address and port explicitly.
    let listener = TcpListener::bind(address).await?; // Establishes the TCP listener to handle incoming requests.
    println!("listening on {}", address);

    axum::serve(listener, app).await?; // Combines the router and the listener, and starts serving HTTP requests.

    Ok(())
}

/// The root handler.
///
/// This function is the handler for the root route of the application. It returns a
/// simple string to indicate that the backend is running.
async fn root_handler(state: Extension<AppState>) -> &'static str {
    let _ = &state.db_client;
    "Attendance portal backend is running"
}
