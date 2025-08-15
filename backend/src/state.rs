//! This module defines the application state.

use mongodb::Client;
use std::sync::Arc;

/// Represents the shared state of the application.
///
/// This struct holds the database client, which is wrapped in an `Arc` to allow
/// for safe sharing across multiple threads.
#[derive(Clone)]
pub struct AppState {
    /// The MongoDB client instance.
    pub db_client: Arc<Client>
}