//! This module defines the routes for the student API.

use crate::error::ErrorType;
use crate::models::student_model::Student;
use crate::state::AppState;
use axum::{Extension, Json};
use mongodb::Collection;

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
/// or an `ErrorType` on failure.
///
/// # Errors
///
/// This function will return an `ErrorType` if:
/// * The roll number is not a positive number (`ErrorType::NegativeRollNumber`).
/// * The email already exists in the database (`ErrorType::EmailAlreadyExists`).
/// * There is an error inserting the student into the database (`ErrorType::ServerError`).
pub async fn add_student(
    state: Extension<AppState>,
    mut student: Json<Student>,
) -> Result<Json<Student>, ErrorType> {
    let collection: Collection<Student> = state
        .db_client
        .database("attendance")
        .collection("students");

    // We use `.0` here because `student` is of type `Json<Student>`, which is a tuple struct.
    // The actual `Student` value is stored in the first (and only) field of the `Json` wrapper,
    // so we access it with `.0`.
    let mut new_student = student.0.clone();
    student.id = None;

    // Check that roll number is positive
    if student.roll_number <= 0 {
        println!("Roll number must be positive");
        return Err(ErrorType::NegativeRollNumber(
            "Roll number must be positive".to_string(),
        ));
    }

    // Check that email does not already exist
    let email_exists = collection
    .find_one(mongodb::bson::doc! { "email": &student.email })
    .await
    .map_err(|err| {
        println!("Error checking for existing email: {:?}", err);
        ErrorType::ServerError("ServerError".to_string())
    })?;
        
    if email_exists.is_some() {
        println!("Email already exists: {}", &student.email);
        return Err(ErrorType::EmailAlreadyExists(
            "Student with email already exists".to_string(),
        ));
    }

    match collection.insert_one(&new_student).await {
        Ok(insert_result) => {
            new_student.id = insert_result.inserted_id.as_object_id();
            Ok(Json(new_student))
        }
        Err(err) => {
            println!("Error inserting student: {:?}", err);
            Err(ErrorType::ServerError("Server Error".to_string()))
        }
    }
}
