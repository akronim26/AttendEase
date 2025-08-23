//! This module defines the routes for the teacher API.

use crate::error::ErrorType;
use crate::models::{class_model::Class, teacher_model::Teacher};
use crate::state::AppState;
use axum::{Extension, Json};
use mongodb::Collection;

/// This function takes the application state and a JSON payload of a teacher as input,
/// and inserts the teacher into the database. If the insertion is successful, the
/// newly inserted teacher with their ID is returned.
///
/// # Arguments
///
/// * `state` - The application state, which contains the database client.
/// * `teacher` - The JSON payload of the teacher to add.
///
/// # Returns
///
/// A `Result` containing a JSON payload of the newly inserted teacher on success,
/// or an `ErrorType` on failure.
///
/// # Errors
///
/// This function will return an `ErrorType` if:
/// * The email already exists in the database (`ErrorType::AlreadyExists`).
/// * The class does not exist (`ErrorType::DoesNotExist`).
/// * There is an error inserting the teacher into the database (`ErrorType::ServerError`).
pub async fn add_teacher(
    Extension(state): Extension<AppState>,
    mut teacher: Json<Teacher>,
) -> Result<Json<Teacher>, ErrorType> {
    let teacher_collection: Collection<Teacher> =
        state.db_client.database("attendance").collection("teacher");

    let mut new_teacher = teacher.0.clone();
    teacher.id = None;

    // Check that email does not already exist
    let email_exists = teacher_collection
        .find_one(mongodb::bson::doc! { "email": &teacher.email })
        .await
        .map_err(|err| {
            println!("Error checking for existing email: {:?}", err);
            ErrorType::ServerError("ServerError".to_string())
        })?;

    if email_exists.is_some() {
        println!("Email already exists: {}", &teacher.email);
        ErrorType::AlreadyExists("Teacher with email already exists".to_string());
    }

    // Check if class exists
    let class_collection: Collection<Class> =
        state.db_client.database("attendance").collection("classes");

    let class_id = teacher.class.clone();

    let class_exist = class_collection
        .find_one(mongodb::bson::doc! { "_id": &class_id })
        .await
        .map_err(|err| {
            println!("Error checking for existing class: {}", err);
            ErrorType::ServerError("Server Error".to_string())
        })?;

    if class_exist.is_none() {
        return Err(ErrorType::DoesNotExist(
            "The class does not exist".to_string(),
        ));
    }

    match teacher_collection.insert_one(&new_teacher).await {
        Ok(insert_result) => {
            new_teacher.id = insert_result.inserted_id.as_object_id();
            Ok(Json(new_teacher))
        }
        Err(err) => {
            println!("Error inserting teacher: {:?}", err);
            Err(ErrorType::ServerError("Server Error".to_string()))
        }
    }
}
