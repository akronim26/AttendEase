//! This module defines the `Class` model.

use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

/// This struct is used to model the data of a class, including its ID and name.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Class {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    /// The name of the class/subject
    pub name: String,
}
