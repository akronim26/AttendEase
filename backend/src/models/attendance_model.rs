//! This module defines the 'Attendance' model.

use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// This struct is used to model the data of attendance, including ID, student_id,
/// class, time, and attendance flag.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attendance {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The reference to Student
    pub student_id: ObjectId,
    /// The class for which attendance is being marked
    pub class: Option<ObjectId>,
    /// The time when attendance was marked
    /// The line below tells the serde how to deserialise the DateTime
    #[serde(with = "chrono::serde::ts_seconds")]
    pub time: DateTime<Utc>,
    /// The flag indicating attendance status (true for present, false for absent)
    pub flag: bool,
}
