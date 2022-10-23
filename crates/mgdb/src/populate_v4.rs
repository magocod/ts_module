use crate::db::connect;
use fake::faker::company::raw::Industry;
use fake::locales::EN;
use fake::Fake;
use mongodb::bson::oid::ObjectId;
use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Category {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SubCategory {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    title: String,
    category: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
struct Tag {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    name: String,
    categories: ObjectId,
    sub_category: Vec<ObjectId>,
}

pub async fn seed() -> Result<(), Box<dyn Error>> {
    let client = connect().await?;
    let db = client.database("dbv4");

    let category_collection = db.collection::<Category>("category");

    let category_id = category_collection
        .insert_one(
            Category {
                id: None,
                title: Industry(EN).fake(),
            },
            None,
        )
        .await?;

    let subcategory_collection = db.collection::<SubCategory>("subcategory");

    let subcategory_id = subcategory_collection
        .insert_one(
            SubCategory {
                id: None,
                title: Industry(EN).fake(),
                category: category_id
                    .inserted_id
                    .as_object_id()
                    .expect("error as_object_id"),
            },
            None,
        )
        .await?;

    let tag_collection = db.collection::<Tag>("tag");

    tag_collection
        .insert_one(
            Tag {
                id: None,
                name: Industry(EN).fake(),
                categories: category_id
                    .inserted_id
                    .as_object_id()
                    .expect("error as_object_id"),
                sub_category: vec![subcategory_id
                    .inserted_id
                    .as_object_id()
                    .expect("error as_object_id")],
            },
            None,
        )
        .await?;

    Ok(())
}
