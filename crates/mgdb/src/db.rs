use mongodb::{options::ClientOptions, Client};
use std::error::Error;

use futures::stream::TryStreamExt;
// use mongodb::bson::oid::ObjectId;
use mongodb::options::FindOptions;
use pluralizer::pluralize;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

// enum GenericDocumentKey {
//     String(String),
//     ObjectId(ObjectId),
// }

// enum SchemaValue<'a> {
//     String(&'a str),
//     VecOfString(Vec<String>),
//     AnotherHashMap(HashMap<&'a str, u32>),
// }

pub type GenericDocument = Map<String, Value>;

#[derive(Debug, Serialize, Deserialize)]
pub struct MongoCollection<T> {
    pub name: String,
    pub docs: Vec<T>,
    pub schema: GenericDocument,
}

pub async fn connect() -> Result<Client, Box<dyn Error>> {
    // Parse a connection string into an options struct.
    let client_options = ClientOptions::parse("mongodb://localhost:27017/?readPreference=primary&appname=MongoDB%20Compass&directConnection=true&ssl=false").await?;

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;

    Ok(client)
}

pub fn get_schema(
    document: GenericDocument,
    schema: &mut GenericDocument,
    collection_names: &Vec<String>,
) {
    for key in document.keys() {
        println!("{}", key);
        match document.get(key) {
            None => {}
            Some(v) => {
                get_type(key.to_string(), v, schema, collection_names);
            }
        }
    }
}

pub fn get_type(
    key: String,
    value: &Value,
    map: &mut GenericDocument,
    collection_names: &Vec<String>,
) {
    // let mut typeName: String = String::new();
    match value {
        Value::Null => {
            println!("null {}", value);
            map.insert(key, Value::String("null".to_string()));
        }
        Value::Bool(_) => {
            println!("bool {}", value);
            map.insert(key, Value::String("bool".to_string()));
        }
        Value::Number(_) => {
            println!("number {}", value);
            map.insert(key.to_string(), Value::String("number".to_string()));
        }
        Value::String(_) => {
            println!("string {}", value);
            map.insert(key.to_string(), Value::String("string".to_string()));
        }
        Value::Array(arr) => {
            println!("array {:#?}", arr);
            match arr.first() {
                None => {}
                Some(val) => {
                    let mut temp_map = Map::new();
                    // get_type("array".to_string(), val, &mut temp_map);
                    get_type(key.to_string(), val, &mut temp_map, collection_names);
                    map.insert(key.to_string(), Value::Object(temp_map));
                }
            }
            // map.insert(key.to_string(), Value::String("array".to_string()));
        }
        Value::Object(ob) => {
            println!("object {:?}", ob);
            match ob.get("$oid") {
                None => {
                    let m = get_map_schema(ob, map, collection_names);
                    map.insert(key.to_string(), Value::Object(m));
                }
                Some(_) => {
                    let mut temp_map = Map::new();
                    temp_map.insert("$oid".to_string(), Value::String("string".to_string()));
                    if key != "_id" {
                        let plural_key = pluralize(key.as_str(), 2, false);
                        println!("pluralize {} -> {}", key, plural_key);
                        if collection_names.contains(&plural_key) {
                            temp_map.insert("$collection".to_string(), Value::String(plural_key));
                        }
                    }
                    map.insert(key.to_string(), Value::Object(temp_map));
                }
            }
        }
    }
}

pub fn get_map_schema(
    document: &Map<String, Value>,
    schema: &mut GenericDocument,
    collection_names: &Vec<String>,
) -> GenericDocument {
    let mut temp_map = Map::new();
    for key in document.keys() {
        println!("{}", key);
        match document.get(key) {
            None => {}
            Some(v) => match v {
                Value::Null => {
                    println!("null {}", v);
                    temp_map.insert(key.to_string(), Value::String("null".to_string()));
                }
                Value::Bool(_) => {
                    println!("bool {}", v);
                    temp_map.insert(key.to_string(), Value::String("bool".to_string()));
                }
                Value::Number(_) => {
                    println!("number {}", v);
                    temp_map.insert(key.to_string(), Value::String("number".to_string()));
                }
                Value::String(_) => {
                    println!("string {}", v);
                    temp_map.insert(key.to_string(), Value::String("string".to_string()));
                }
                Value::Array(arr) => {
                    println!("array {:#?}", arr);
                    match arr.first() {
                        None => {}
                        Some(val) => {
                            let mut sub_map = Map::new();
                            // get_type("array".to_string(), val, &mut sub_map);
                            get_type(key.to_string(), val, &mut sub_map, collection_names);
                            temp_map.insert(key.to_string(), Value::Object(sub_map));
                        }
                    }
                    // temp_map.insert(key.to_string(), Value::String("array".to_string()));
                }
                Value::Object(ob) => {
                    println!("object {:?}", ob);

                    match ob.get("$oid") {
                        None => {
                            let m = get_map_schema(ob, schema, collection_names);
                            temp_map.insert(key.to_string(), Value::Object(m));
                        }
                        Some(_) => {
                            let mut sub_map = Map::new();
                            sub_map.insert("$oid".to_string(), Value::String("string".to_string()));
                            if key != "_id" {
                                let plural_key = pluralize(key.as_str(), 2, false);
                                println!("pluralize {} -> {}", key, plural_key);
                                if collection_names.contains(&plural_key) {
                                    sub_map.insert("$collection".to_string(), Value::String(plural_key));
                                }
                            }
                            temp_map.insert(key.to_string(), Value::Object(sub_map));
                        }
                    }
                }
            },
        }
    }
    temp_map
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
    for collection_name in collection_names.clone() {
        // println!("collection: {}", collection_name);

        let collection = db.collection::<GenericDocument>(collection_name.as_str());

        let find_options = FindOptions::builder().limit(1).build();
        let mut cursor = collection.find(None, find_options).await?;

        let mut col = MongoCollection::<GenericDocument> {
            name: collection_name,
            docs: vec![],
            schema: Map::new(),
        };

        // Iterate over the results of the cursor.
        while let Some(document) = cursor.try_next().await? {
            // println!("document: {:#?}", document);
            get_schema(document, &mut col.schema, &collection_names);
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
