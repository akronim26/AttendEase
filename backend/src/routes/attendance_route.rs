//! This module defines the routes for the attendance API.

use crate::models::attendance_model::Attendance;
use crate::models::student_model::Student;
use crate::state::AppState;
use crate::{error::ErrorType, models::class_model::Class};
use axum::{Extension, Json};
use chrono::Utc;
use mongodb::Collection;

/// This function takes the application state and a JSON payload of attendance as input,
/// and marks the attendance of the student. If the process is successful, the
/// attendance details are returned.
///
/// # Arguments
///
/// * `state` - The application state, which contains the database client.
/// * `attendance_details` - The JSON payload of the attendance to add.
///
/// # Returns
///
/// A `Result` containing a JSON payload of the newly inserted attendance on success,
/// or an `ErrorType` on failure.
///
/// # Errors
///
/// This function will return an `ErrorType` if:
/// * The class does not exist (`ErrorType::DoesNotExist`).
/// * The student does not exist (`ErrorType::DoesNotExist`).
/// * There is an error marking the attendance (`ErrorType::ServerError`).
pub async fn mark_attendance(
    Extension(state): Extension<AppState>,
    mut attendance_details: Json<Attendance>,
) -> Result<Json<Attendance>, ErrorType> {
    let attendance_collection: Collection<Attendance> =
        state.db_client.database("attendance").collection("records");

    attendance_details.id = None;
    attendance_details.time = Utc::now();
    attendance_details.flag = true;
    let mut new_details = attendance_details.0.clone();

    // Check if student exists
    let student_collection: Collection<Student> = state
        .db_client
        .database("attendance")
        .collection("students");

    let student_id = attendance_details.student_id;
    let student_exist = student_collection
        .find_one(mongodb::bson::doc! { "_id": &student_id })
        .await
        .map_err(|err| {
            println!("Error checking for existing student: {}", err);
            ErrorType::ServerError("Server Error".to_string())
        })?;

    if student_exist.is_none() {
        return Err(ErrorType::DoesNotExist(
            "The student does not exist".to_string(),
        ));
    }

    // Check if class exists
    let class_collection: Collection<Class> =
        state.db_client.database("attendance").collection("classes");

    let class_id = attendance_details.class.clone();

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

    match attendance_collection.insert_one(&new_details).await {
        Ok(insert_result) => {
            new_details.id = insert_result.inserted_id.as_object_id();
            Ok(Json(new_details))
        }
        Err(err) => {
            println!("Error inserting attendance: {:?}", err);
            Err(ErrorType::ServerError("Server Error".to_string()))
        }
    }
}
