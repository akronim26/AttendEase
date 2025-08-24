//! This module defines the routes for the class API.

use crate::error::ErrorType;
use crate::models::class_model::Class;
use crate::state::AppState;
use axum::{Extension, Json};
use mongodb::Collection;
use tokio_stream::StreamExt;

/// This function takes the application state and returns all the classes.
///
/// # Arguments
///
/// * `state` - The application state, which contains the database client.
///
/// # Returns
///
/// A `Result` containing a JSON payload of all classes on success,
/// or an `ErrorType` on failure.
///
/// # Errors
///
/// This function will return an `ErrorType` if:
/// * There is an error fetching the classes from the database (`ErrorType::ServerError`).
pub async fn get_classes(
    Extension(state): Extension<AppState>,
) -> Result<Json<Vec<Class>>, ErrorType> {
    let class_collection: Collection<Class> =
        state.db_client.database("attendance").collection("classes");

    let cursor_result = class_collection
        .find(mongodb::bson::doc! {})
        .await
        .map_err(|err| {
            println!("Error fetching the classes: {}", err);
            ErrorType::ServerError("Server Error".to_string())
        });

    let mut classes = Vec::new();
    match cursor_result {
        Ok(mut cursor) => {
            while let Ok(Some(class)) = cursor.try_next().await.map_err(|err| {
                println!("Error fetching the classes: {}", err);
                ErrorType::ServerError("Server Error".to_string())
            }) {
                classes.push(class);
            }
        }
        Err(_) => {
            println!("Error fetching the classes");
        }
    }

    Ok(Json(classes))
}

/// This function takes the application state and a JSON payload of a class as input,
/// and inserts the class into the database. If the insertion is successful, the
/// newly inserted class with their ID is returned.
///
/// # Arguments
///
/// * `state` - The application state, which contains the database client.
/// * `class` - The JSON payload of the class to add.
///
/// # Returns
///
/// A `Result` containing a JSON payload of the newly inserted class on success,
/// or an `ErrorType` on failure.
///
/// # Errors
///
/// This function will return an `ErrorType` if:
/// * The class already exists in the database (`ErrorType::AlreadyExists`).
/// * There is an error inserting the class into the database (`ErrorType::ServerError`).
pub async fn add_class(
    Extension(state): Extension<AppState>,
    mut class: Json<Class>,
) -> Result<Json<Class>, ErrorType> {
    let class_collection: Collection<Class> =
        state.db_client.database("attendance").collection("classes");

    class.id = None;

    // Check if class already exists
    let class_exist = class_collection
        .find_one(mongodb::bson::doc! { "name": &class.name })
        .await
        .map_err(|err| {
            println!("Error checking for existing class: {}", err);
            ErrorType::ServerError("Server Error".to_string())
        })?;

    if class_exist.is_some() {
        return Err(ErrorType::AlreadyExists(
            "The class already exists".to_string(),
        ));
    }

    match class_collection.insert_one(&*class).await {
        Ok(insert_result) => {
            class.id = insert_result.inserted_id.as_object_id();
            let new_class = class.0.clone();
            Ok(Json(new_class))
        }
        Err(err) => {
            println!("Error inserting class: {:?}", err);
            Err(ErrorType::ServerError("Server Error".to_string()))
        }
    }
}
