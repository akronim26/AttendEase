use mongodb::Client;
use std::sync::Arc;

/// Represents the shared state of the application.
///
/// It's marked with `#[derive(Clone)]` so `axum` can create a copy for each handler.
/// This works by cloning each field inside the struct.
///
/// `db_client: Arc<Client>`: A smart pointer to the shared MongoDB client.
/// 
/// Cloning an `Arc` is cheap. It doesn't duplicate the
/// database client; it just creates another pointer to the same instance and
/// increments a reference counter.
/// 
/// `Arc` prevents data races at compile time by
/// only allowing shared, read-only access. Code that tries to mutate the
/// client through the `Arc` will not compile.
/// 
/// The `mongodb::Client` itself is a thread-safe object
/// that manages a connection pool. It uses interior mutability to ensure that
/// concurrent tasks can safely request and use database connections without
/// interfering with each other.
#[derive(Clone)]
pub struct AppState {
    pub db_client: Arc<Client>,
}
