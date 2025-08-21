//! This module defines the 'Attendance' model.

use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// This struct is used to model the data of attendance, including ID, student_id,
/// time, and attendanceFlag.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attendance {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    // The reference to Student
    pub student_id: ObjectId,
    // The time of attendance
    // The line below tells the serde to deserialise the DateTime
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    // The subject of the attendance
    pub subject: String
}
