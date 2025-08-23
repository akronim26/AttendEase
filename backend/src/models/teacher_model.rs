//! This module defines the `Teacher` model.

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// This struct is used to model the data of a teacher, including their ID, name,
/// email, and assigned class.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Teacher {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The name of the teacher
    pub name: String,
    /// The email of the teacher
    pub email: String,
    /// The class taken by the teacher
    pub class: Option<ObjectId>,
}
