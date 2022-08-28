use serde::{Deserialize, Serialize};

pub const COLL_USERS: &str = "users";
pub const COLL_TOKENS: &str = "tokens";

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // pub id: Option<ObjectId>,
    pub token: String,
    // pub user: ObjectId,
}
