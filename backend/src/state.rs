use std::sync::Arc;
use mongodb::Client;

#[derive(Clone)] // This line automatically creates a clone of the app state whenever any functionality
// demands it
pub struct AppState {
    pub db_client: Arc<Client>,
}