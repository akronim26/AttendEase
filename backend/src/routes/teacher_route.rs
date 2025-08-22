//! This module defines the routes for the teacher API.

use crate::error::ErrorType;
use crate::models::teacher_model::Teacher;
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
/// * `teacher` - The JSON payload of the student to add.
///
/// # Returns
///
/// A `Result` containing a JSON payload of the newly inserted teacher on success,
/// or an `ErrorType` on failure.
///
/// # Errors
///
/// This function will return an `ErrorType` if:
/// * The email already exists in the database (`ErrorType::EmailAlreadyExists`).
/// * The subject is empty (`ErrorType::SubjectEmpty`).
/// * There is an error inserting the student into the database (`ErrorType::ServerError`).
pub async fn add_teacher(
    state: Extension<AppState>,
    mut teacher: Json<Teacher>,
) -> Result<Json<Teacher>, ErrorType> {
    let collection: Collection<Teacher> = state
        .db_client
        .database("attendance")
        .collection("teacher");

    let mut new_teacher = teacher.0.clone();
    teacher.id = None;

    // Check that email does not already exist
    let email_exists = collection
    .find_one(mongodb::bson::doc! { "email": &teacher.email })
    .await
    .map_err(|err| {
        println!("Error checking for existing email: {:?}", err);
        ErrorType::ServerError("ServerError".to_string())
    })?;
        
    if email_exists.is_some() {
        println!("Email already exists: {}", &teacher.email);
        return Err(ErrorType::EmailAlreadyExists(
            "Teacher with email already exists".to_string(),
        ));
    }

    match collection.insert_one(&new_teacher).await {
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
