//! This module defines the custom error types for the application.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// Represents the possible errors that can occur in the application.
#[derive(Debug)]
pub enum ErrorType {
    /// Returned when trying to create a student with an email that already exists.
    EmailAlreadyExists(String),
    /// Returned when trying to create a student with a negative roll number.
    NegativeRollNumber(String),
    /// Returned when trying to mark attendance with empty subject.
    SubjectEmpty(String),
    /// Returned when the subject of attendance is not in the curriculum.
    SubjectDoesNotExist(String),
    /// Returned for generic server errors.
    ServerError(String),
}

impl IntoResponse for ErrorType {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ErrorType::EmailAlreadyExists(msg) => (StatusCode::CONFLICT, msg),
            ErrorType::NegativeRollNumber(msg) => (StatusCode::BAD_REQUEST, msg),
            ErrorType::SubjectEmpty(msg) => (StatusCode::BAD_REQUEST, msg),
            ErrorType::ServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            ErrorType::SubjectDoesNotExist(msg) => (StatusCode::BAD_REQUEST, msg),
        };

        let body = serde_json::json!({
            "error": error_message
        });
        (status, Json(body)).into_response()
    }
}
