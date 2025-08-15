//! This module defines the routes for the student API.

use axum::{Json, Extension, http::StatusCode};
use mongodb::Collection;
use crate::state::AppState;
use crate::models::student_model::Student;

/// Adds a new student to the database.
///
/// This function takes the application state and a JSON payload of a student as input,
/// and inserts the student into the database. If the insertion is successful, the
/// newly inserted student with their ID is returned.
///
/// # Arguments
///
/// * `state` - The application state, which contains the database client.
/// * `student` - The JSON payload of the student to add.
///
/// # Returns
///
/// A `Result` containing a JSON payload of the newly inserted student on success,
/// or a `StatusCode` on failure.
///
/// # Errors
///
/// This function will return an error if:
/// * The roll number is not a positive number.
/// * The email already exists in the database.
/// * There is an error inserting the student into the database.
pub async fn add_student(state: Extension<AppState>, student:Json<Student>) -> Result<Json<Student>, StatusCode> {
    let collection: Collection<Student> = state.db_client.database("attendance").collection("students");

    // We use `.0` here because `student` is of type `Json<Student>`, which is a tuple struct.
    // The actual `Student` value is stored in the first (and only) field of the `Json` wrapper,
    // so we access it with `.0`.
    let mut new_student = student.0.clone();
    new_student.id = None;

    // Check that roll number is positive
    if let Ok(roll_num) = new_student.roll_number.parse::<i64>() {
        if roll_num <= 0 {
            println!("Roll number must be positive");
            return Err(StatusCode::BAD_REQUEST);
        }
    } else {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check that email does not already exist
    let email_exists = collection
        .find_one(mongodb::bson::doc! { "email": &new_student.email })
        .await
        .map_err(|err| {
            println!("Error checking for existing email: {:?}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if email_exists.is_some() {
        println!("Email already exists: {}", &new_student.email);
        return Err(StatusCode::CONFLICT);
    }

    match collection.insert_one(&new_student).await {
        Ok(insert_result) => {
            println!("Successfully inserted student");
            let mut inserted_student = new_student.clone();
            inserted_student.id = insert_result.inserted_id.as_object_id();
            Ok(Json(inserted_student))
        },
        Err(err) => {
            println!("Error inserting student: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}