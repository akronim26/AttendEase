//! This module defines the routes for the attendance API.

use crate::models::attendance_model::Attendance;
use crate::models::student_model::Student;
use crate::state::AppState;
use crate::{error::ErrorType, models::class_model::Class};
use axum::{Extension, Json, extract::Path};
use chrono::Utc;
use mongodb::{Collection, bson::oid::ObjectId};
use tokio_stream::StreamExt;

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

    let class_id = attendance_details.class_id.clone();

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

/// This function takes the application state and a student ID as input,
/// and searches the record for the attendance for a student in the database.
/// If the record is found, the JSON payload of student's attendance
/// is returned.
///
/// # Arguments
///
/// * `state` - The application state, which contains the database client.
/// * `student_id` - The ID of the student to search.
///
/// # Returns
///
/// A `Result` containing a JSON payload of the student's attendance on success,
/// or an `ErrorType` on failure.
///
/// # Errors
///
/// This function will return an `ErrorType` if:
/// * The student record is not found (`ErrorType::DoesNotExist`).
/// * There is an error searching the student's attendance in the database (`ErrorType::ServerError`).
pub async fn get_attendance_by_student(
    Extension(state): Extension<AppState>,
    Path(student_id): Path<ObjectId>,
) -> Result<Json<Vec<Attendance>>, ErrorType> {
    let collection: Collection<Attendance> =
        state.db_client.database("attendance").collection("records");

    let cursor_result = collection
        .find(mongodb::bson::doc! {"student_id": student_id})
        .await
        .map_err(|err| {
            println!("Error fetching the attendance: {}", err);
            ErrorType::ServerError("Server Error".to_string())
        });

    let mut attendances = Vec::new();

    match cursor_result {
        Ok(mut cursor) => {
            while let Ok(Some(attendance)) = cursor.try_next().await.map_err(|err| {
                println!("Error fetching the attendance: {}", err);
                ErrorType::ServerError("Server Error".to_string())
            }) {
                attendances.push(attendance);
            }
        }
        Err(_) => {
            println!("Error fetching the attendances");
        }
    };

    Ok(Json(attendances))
}

/// This function takes the application state and a class ID as input,
/// and searches the record for the attendance for a class in the database.
/// If the record is found, the JSON payload of class' attendance
/// is returned.
///
/// # Arguments
///
/// * `state` - The application state, which contains the database client.
/// * `class_id` - The ID of the class to search.
///
/// # Returns
///
/// A `Result` containing a JSON payload of the class' attendance on success,
/// or an `ErrorType` on failure.
///
/// # Errors
///
/// This function will return an `ErrorType` if:
/// * The class record is not found (`ErrorType::DoesNotExist`).
/// * There is an error searching the class attendance in the database (`ErrorType::ServerError`).
pub async fn get_attendance_by_class(
    Extension(state): Extension<AppState>,
    Path(class_id): Path<ObjectId>,
) -> Result<Json<Vec<Attendance>>, ErrorType> {
    let collection: Collection<Attendance> =
        state.db_client.database("attendance").collection("records");

    let cursor_result = collection
        .find(mongodb::bson::doc! {"class_id": class_id})
        .await
        .map_err(|err| {
            println!("Error fetching the attendance: {}", err);
            ErrorType::ServerError("Server Error".to_string())
        });

    let mut attendances = Vec::new();

    match cursor_result {
        Ok(mut cursor) => {
            while let Ok(Some(attendance)) = cursor.try_next().await.map_err(|err| {
                println!("Error fetching the attendance: {}", err);
                ErrorType::ServerError("Server Error".to_string())
            }) {
                attendances.push(attendance);
            }
        }
        Err(_) => {
            println!("Error fetching the attendances");
        }
    };

    Ok(Json(attendances))
}
