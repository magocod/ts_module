use serde::{Deserialize, Serialize};

use fake::faker::internet::raw::{Password, SafeEmail};
use fake::faker::name::raw::{FirstName, LastName};
use fake::locales::EN;
use fake::Fake;

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

impl User {
    /// generate random data
    pub fn factory() -> Self {
        Self {
            // id: None,
            name: FirstName(EN).fake::<String>() + " " + LastName(EN).fake(),
            email: SafeEmail(EN).fake(),
            password: Password(EN, 6..8).fake(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    // #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    // pub id: Option<ObjectId>,
    pub token: String,
    // pub user: ObjectId,
}
