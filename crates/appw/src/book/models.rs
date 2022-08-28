use serde::{Deserialize, Serialize};

pub const COLL_BOOKS: &str = "books";

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // pub id: Option<ObjectId>,
    pub title: String,
    pub author: String,
}
