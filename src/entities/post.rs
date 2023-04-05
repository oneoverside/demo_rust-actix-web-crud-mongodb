use chrono::{DateTime, Utc};
use ::serde::{Deserialize, Serialize};
use bson::oid::ObjectId;

#[derive(Deserialize, Serialize, Debug)]
pub struct Post {
    pub _id: ObjectId,
    pub title: String,
    pub connect: String,
    pub author: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}