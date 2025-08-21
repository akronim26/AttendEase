//! This module defines the routes for the attendance API.

use crate::error::ErrorType;
use crate::models::attendance_model::Attendance;
use crate::state::AppState;
use axum::{Extension, Json};
use chrono::Utc;
use mongodb::Collection;

/// Accepts the application state and a JSON payload containing `student_id` and
/// `attendance_flag`. The server sets `id` to `None` for insert and stamps the
/// current UTC `time`.
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
/// * The subject is empty (`ErrorType::SubjectEmpty`).
/// * There is an error marking the attendance (`ErrorType::ServerError`).
pub async fn mark_attendance(
    state: Extension<AppState>,
    mut attendance_details: Json<Attendance>,
) -> Result<Json<Attendance>, ErrorType> {
    let collection: Collection<Attendance> =
        state.db_client.database("attendance").collection("records");

    attendance_details.id = None;
    attendance_details.time = Utc::now();
    let mut new_details = attendance_details.0.clone();

    if new_details.subject.is_empty() {
        println!("Subject cannot be empty");
        return Err(ErrorType::SubjectEmpty(
            "Subject cannot be empty".to_string(),
        ));
    }

    match collection.insert_one(&new_details).await {
        Ok(insert_result) => {
            new_details.id = insert_result.inserted_id.as_object_id();
            Ok(Json(new_details))
        }
        Err(err) => {
            println!("Error inserting student: {:?}", err);
            Err(ErrorType::ServerError("Server Error".to_string()))
        }
    }
}
