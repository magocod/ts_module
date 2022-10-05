use mongodb::{options::ClientOptions, Client};
use std::collections::HashMap;
use std::error::Error;

use futures::stream::TryStreamExt;
// use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOptions;

use serde::{Deserialize, Serialize};
use serde_json::Value;

// enum GenericDocumentKey {
//     String(String),
//     ObjectId(ObjectId),
// }

pub type GenericDocument = HashMap<String, Value>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongoCollection<T> {
    pub name: String,
    pub docs: Vec<T>,
    pub schema: HashMap<String, String>,
}

pub async fn connect() -> Result<Client, Box<dyn Error>> {
    // Parse a connection string into an options struct.
    let client_options = ClientOptions::parse("mongodb://localhost:27017/?readPreference=primary&appname=MongoDB%20Compass&directConnection=true&ssl=false").await?;

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    Ok(client)
}

pub async fn explore() -> Result<Vec<MongoCollection<GenericDocument>>, Box<dyn Error>> {
    let client = connect().await?;
    // for db_name in client.list_database_names(None, None).await? {
    //     println!("{}", db_name);
    // }

    // Get a handle to a database.
    let db = client.database("actix");

    let collection_names = db.list_collection_names(None).await?;

    let mut collection_map: Vec<MongoCollection<GenericDocument>> = vec![];

    // List the names of the collections in that database.
    for collection_name in collection_names {
        // println!("collection: {}", collection_name);

        let collection = db.collection::<GenericDocument>(collection_name.as_str());

        let find_options = FindOptions::builder().limit(1).build();
        let mut cursor = collection.find(None, find_options).await?;

        let mut col = MongoCollection::<GenericDocument> {
            name: collection_name,
            docs: vec![],
            schema: HashMap::new(),
        };

        // Iterate over the results of the cursor.
        while let Some(document) = cursor.try_next().await? {
            // println!("document: {:#?}", document);
            for key in document.keys() {
                println!("{}", key);
                match document.get(key) {
                    None => {}
                    Some(v) => {
                        match v {
                            Value::Null => {
                                println!("null {}", v);
                                col.schema.insert(key.to_string(), "null".to_string());
                            }
                            Value::Bool(_) => {
                                println!("bool {}", v);
                                col.schema.insert(key.to_string(), "bool".to_string());
                            }
                            Value::Number(_) => {
                                println!("number {}", v);
                                col.schema.insert(key.to_string(), "number".to_string());
                            }
                            Value::String(_) => {
                                println!("string {}", v);
                                col.schema.insert(key.to_string(), "string".to_string());
                            }
                            Value::Array(_) => {
                                println!("array {}", v);
                                col.schema.insert(key.to_string(), "array".to_string());
                            }
                            Value::Object(ob) => {
                                println!("object {:?}", ob);
                                col.schema.insert(key.to_string(), "object".to_string());
                            }
                        }
                    }
                }
            }
        }

        collection_map.push(col);
    }

    Ok(collection_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn explore_db() {
        let _ = explore().await.expect("fail explore");
        assert_eq!(1, 1);
    }
}