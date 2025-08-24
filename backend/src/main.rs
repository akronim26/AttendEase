//! This is the main entry point of the application.

mod db;
mod state;
mod routes {
    pub mod attendance_route;
    pub mod class_route;
    pub mod student_route;
    pub mod teacher_route;
}
mod models {
    pub mod attendance_model;
    pub mod class_model;
    pub mod student_model;
    pub mod teacher_model;
}
mod error;

use crate::routes::{
    attendance_route::{get_attendance_by_class, get_attendance_by_student, mark_attendance},
    class_route::{add_class, get_classes},
    student_route::{add_student, get_student},
    teacher_route::{add_teacher, get_teacher},
};
use crate::state::AppState;
use axum::{
    Extension, Router,
    routing::{get, post},
};
use dotenvy::dotenv;
use error::ErrorType;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::TcpListener;

/// This function initializes the application by loading the environment variables,
/// connecting to the database, creating the application state, and starting the
/// HTTP server.
#[tokio::main]
pub async fn main() -> Result<(), ErrorType> {
    dotenv().expect(".env does not exist");

    let client = db::connect_to_database().await.map_err(|err| {
        println!("Error starting the server: {}", err);
        ErrorType::ServerStartingError("Server failed to start".to_string())
    })?;
    let shared_client = Arc::new(client);

    let app_state = AppState {
        db_client: shared_client.clone(),
    };

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/attendance/mark", post(mark_attendance))
        .route(
            "/attendance/students/{student_id}",
            get(get_attendance_by_student),
        )
        .route(
            "/attendance/classes/{class_id}",
            get(get_attendance_by_class),
        )
        .route("/classes", get(get_classes))
        .route("/classes/add", post(add_class))
        .route("/students/add", post(add_student))
        .route("/students/{student_id}", get(get_student))
        .route("/teacher/add", post(add_teacher))
        .route("/teacher{teacher_id}", get(get_teacher))
        .layer(Extension(app_state)); // Injects the application state into all routes.

    let address = SocketAddr::from(([127, 0, 0, 1], 3000)); // Defines the IP address and port explicitly.
    let listener = TcpListener::bind(address).await.map_err(|err| {
        println!("Error binding to address: {}", err);
        ErrorType::ServerStartingError("Server failed to bind".to_string())
    })?; // Establishes the TCP listener to handle incoming requests.

    println!("listening on {}", address);
    
    axum::serve(listener, app).await.map_err(|err| {
        println!("Error serving: {}", err);
        ErrorType::ServerStartingError("Server failed to serve".to_string())
    })?; // Combines the router and the listener, and starts serving HTTP requests.


    Ok(())
}

/// This function is the handler for the root route of the application. It returns a
/// simple string to indicate that the backend is running.
///
/// # Arguments
///
/// * `state` - The application state, which contains the database client.
///
/// # Returns
///
/// A static string message 'Attendance portal backend is running'.
async fn root_handler(Extension(state): Extension<AppState>) -> &'static str {
    let _ = &state.db_client;
    "Attendance portal backend is running"
}
