//! This module defines the `Student` model.

use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

/// Represents a student in the database.
///
/// This struct is used to model the data of a student, including their ID, name,
/// email, and roll number. It is used for both serialization and deserialization.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Student {
    /// The unique identifier of the student.
    ///
    /// This field is an `Option<ObjectId>`, which allows it to be `None` when creating
    /// a new student, and `Some(ObjectId)` when retrieving a student from the database.
    ///
    /// The `#[serde(rename = "_id", skip_serializing_if = "Option::is_none")]` attribute
    /// tells Serde to serialize and deserialize this field as `_id` (matching MongoDB's
    /// convention), and to skip this field when serializing if it is `None`.
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The name of the student.
    pub name: String,
    /// The email of the student.
    pub email: String,
    /// The roll number of the student.
    pub roll_number: String,
}