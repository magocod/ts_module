use crate::db::connect;
use crate::db_common::DBV1;
use fake::faker::address::raw::CountryName;
use fake::faker::boolean::raw::Boolean;
use fake::faker::company::raw::Industry;
use fake::faker::internet::raw::{Password, SafeEmail};
use fake::faker::lorem::raw::Word;
use fake::faker::name::raw::Name;
use fake::locales::EN;
use fake::uuid::UUIDv4;
use fake::Fake;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use rand::Rng;
use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Chapter {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    title: String,
    is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Preview {
    content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Book {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    title: String,
    author: String,
    preview: Preview,
    chapters: Vec<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Country {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    name: String,
    code: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    name: String,
    email: String,
    password: String,
    roles: Vec<i32>,
    date: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    token: String,
    user: ObjectId,
    date: DateTime,
}
#[derive(Debug, Serialize, Deserialize)]
struct Publication {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    date: DateTime,
    book_data: Book,
    book: ObjectId,
    tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Profile {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    user: ObjectId,
    country: ObjectId,
    books: Vec<ObjectId>,
    publications: Vec<Publication>,
}

pub async fn seed() -> Result<(), Box<dyn Error>> {
    let client = connect().await?;
    let db = client.database(DBV1);

    let chapter_collection = db.collection::<Chapter>("chapters");

    let chapter_id = chapter_collection
        .insert_one(
            Chapter {
                id: None,
                title: Industry(EN).fake(),
                is_active: Boolean(EN, 2).fake(),
            },
            None,
        )
        .await?;

    let book_collection = db.collection::<Book>("books");

    let book_data = Book {
        id: None,
        title: Industry(EN).fake(),
        author: Name(EN).fake(),
        preview: Preview {
            content: Word(EN).fake(),
        },
        chapters: vec![chapter_id
            .inserted_id
            .as_object_id()
            .expect("error as_object_id")],
    };

    let book_id = book_collection.insert_one(book_data.clone(), None).await?;

    let country_collection = db.collection::<Country>("countries");

    let country_id = country_collection
        .insert_one(
            Country {
                id: None,
                name: CountryName(EN).fake(),
                code: rand::thread_rng().gen_range(0..100),
            },
            None,
        )
        .await?;

    let user_collection = db.collection::<User>("users");

    let user_id = user_collection
        .insert_one(
            User {
                id: None,
                name: Industry(EN).fake(),
                email: SafeEmail(EN).fake(),
                password: Password(EN, 5..10).fake(),
                roles: vec![
                    rand::thread_rng().gen_range(0..100),
                    rand::thread_rng().gen_range(0..100),
                ],
                date: DateTime::now(),
            },
            None,
        )
        .await?;

    let token_collection = db.collection::<Token>("tokens");

    token_collection
        .insert_one(
            Token {
                id: None,
                token: UUIDv4.fake(),
                user: user_id
                    .inserted_id
                    .as_object_id()
                    .expect("error as_object_id"),
                date: DateTime::now(),
            },
            None,
        )
        .await?;

    let profile_collection = db.collection::<Profile>("profiles");

    profile_collection
        .insert_one(
            Profile {
                id: None,
                user: user_id
                    .inserted_id
                    .as_object_id()
                    .expect("error as_object_id"),
                country: country_id
                    .inserted_id
                    .as_object_id()
                    .expect("error as_object_id"),
                books: vec![book_id
                    .inserted_id
                    .as_object_id()
                    .expect("error as_object_id")],
                publications: vec![Publication {
                    id: None,
                    date: DateTime::now(),
                    book_data,
                    book: book_id
                        .inserted_id
                        .as_object_id()
                        .expect("error as_object_id"),
                    tags: vec![Word(EN).fake(), Word(EN).fake()],
                }],
            },
            None,
        )
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connect;
    use crate::db_common::DBV1;

    #[tokio::test]
    async fn explorer_explore_db_schema() {
        let client = connect().await.expect("error connect mongodb");
        let db = client.database(DBV1);
        let collection_names = db
            .list_collection_names(None)
            .await
            .expect("fail get collections");

        seed().await.expect("fail seed");

        assert_eq!(collection_names.len(), 6);
    }
}
