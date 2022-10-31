use crate::db::{explore_object_id, FinderRelationships, GenericDocument, MongodbCollection};
use crate::db_common::DBV1;
use futures::TryStreamExt;
use mongodb::options::FindOptions;
use mongodb::Database;
use serde_json::{Map, Value};
use std::error::Error;
use std::{fs, io};

#[derive(Clone)]
pub struct Explorer<'a> {
    collection_names: Vec<String>,
    mongodb_collection: Vec<MongodbCollection<GenericDocument>>,
    finder_relationships: &'a FinderRelationships,
}

#[derive(Debug)]
pub struct Cache<'a> {
    pub collection_names: &'a Vec<String>,
    pub mongodb_collection: &'a Vec<MongodbCollection<GenericDocument>>,
}

#[derive(Debug)]
pub struct CacheCopy {
    pub collection_names: Vec<String>,
    pub mongodb_collection: Vec<MongodbCollection<GenericDocument>>,
}

impl<'a> Explorer<'a> {
    pub fn new(finder_relationships: Option<&'a FinderRelationships>) -> Self {
        let f = match finder_relationships {
            None => &explore_object_id,
            Some(func) => func,
        };
        Self {
            collection_names: vec![],
            finder_relationships: f,
            mongodb_collection: vec![],
        }
    }

    fn get_schema(&self, document: GenericDocument, schema: &mut GenericDocument) {
        for key in document.keys() {
            // println!("{}", key);
            match document.get(key) {
                None => {}
                Some(v) => {
                    self.get_type(key.to_string(), v, schema);
                }
            }
        }
    }

    pub fn get_type(&self, key: String, value: &Value, map: &mut GenericDocument) {
        // let mut typeName: String = String::new();
        match value {
            Value::Null => {
                // println!("null {}", value);
                map.insert(key, Value::String("null".to_string()));
            }
            Value::Bool(_) => {
                // println!("bool {}", value);
                map.insert(key, Value::String("bool".to_string()));
            }
            Value::Number(_) => {
                // println!("number {}", value);
                map.insert(key.to_string(), Value::String("number".to_string()));
            }
            Value::String(_) => {
                // println!("string {}", value);
                map.insert(key.to_string(), Value::String("string".to_string()));
            }
            Value::Array(arr) => {
                // println!("array {:#?}", arr);
                match arr.first() {
                    None => {}
                    Some(val) => {
                        let mut temp_map = Map::new();
                        // get_type("array".to_string(), val, &mut temp_map);
                        temp_map.insert("array".to_string(), Value::String("[]".to_string()));
                        self.get_type(key.to_string(), val, &mut temp_map);
                        map.insert(key.to_string(), Value::Object(temp_map));
                    }
                }
                // map.insert(key.to_string(), Value::String("array".to_string()));
            }
            Value::Object(ob) => {
                // println!("object {:?}", ob);
                match ob.get("$oid") {
                    None => {
                        let m = self.get_map_schema(ob, map);
                        map.insert(key.to_string(), Value::Object(m));
                    }
                    Some(_) => {
                        let mut temp_map = Map::new();
                        temp_map.insert("$oid".to_string(), Value::String("string".to_string()));
                        let f = self.finder_relationships;
                        match f(key.as_str(), &self.collection_names) {
                            Some(val) => {
                                temp_map.insert("collection".to_string(), Value::String(val));
                            }
                            None => {}
                        }
                        map.insert(key.to_string(), Value::Object(temp_map));
                    }
                }
            }
        }
    }

    pub fn get_map_schema(
        &self,
        document: &Map<String, Value>,
        schema: &mut GenericDocument,
    ) -> GenericDocument {
        let mut temp_map = Map::new();
        for key in document.keys() {
            // println!("{}", key);
            match document.get(key) {
                None => {}
                Some(v) => match v {
                    Value::Null => {
                        // println!("null {}", v);
                        temp_map.insert(key.to_string(), Value::String("null".to_string()));
                    }
                    Value::Bool(_) => {
                        // println!("bool {}", v);
                        temp_map.insert(key.to_string(), Value::String("bool".to_string()));
                    }
                    Value::Number(_) => {
                        // println!("number {}", v);
                        temp_map.insert(key.to_string(), Value::String("number".to_string()));
                    }
                    Value::String(_) => {
                        // println!("string {}", v);
                        temp_map.insert(key.to_string(), Value::String("string".to_string()));
                    }
                    Value::Array(arr) => {
                        // println!("array {:#?}", arr);
                        match arr.first() {
                            None => {}
                            Some(val) => {
                                let mut sub_map = Map::new();
                                // get_type("array".to_string(), val, &mut sub_map);
                                sub_map
                                    .insert("array".to_string(), Value::String("[]".to_string()));
                                self.get_type(key.to_string(), val, &mut sub_map);
                                temp_map.insert(key.to_string(), Value::Object(sub_map));
                            }
                        }
                        // temp_map.insert(key.to_string(), Value::String("array".to_string()));
                    }
                    Value::Object(ob) => {
                        // println!("object {:?}", ob);

                        match ob.get("$oid") {
                            None => {
                                let m = self.get_map_schema(ob, schema);
                                temp_map.insert(key.to_string(), Value::Object(m));
                            }
                            Some(_) => {
                                let mut sub_map = Map::new();
                                sub_map.insert(
                                    "$oid".to_string(),
                                    Value::String("string".to_string()),
                                );
                                let f = self.finder_relationships;
                                match f(key, &self.collection_names) {
                                    Some(val) => {
                                        sub_map
                                            .insert("collection".to_string(), Value::String(val));
                                    }
                                    None => {}
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

    pub async fn explore(
        &mut self,
        db: &Database,
    ) -> Result<&Vec<MongodbCollection<GenericDocument>>, Box<dyn Error>> {
        self.collection_names = db.list_collection_names(None).await?;

        let mut collection_map: Vec<MongodbCollection<GenericDocument>> = vec![];

        // List the names of the collections in that database.
        for collection_name in self.collection_names.clone() {
            // println!("collection: {}", collection_name);

            let collection = db.collection::<GenericDocument>(collection_name.as_str());

            let find_options = FindOptions::builder().limit(1).build();
            let mut cursor = collection.find(None, find_options).await?;

            let mut col = MongodbCollection::<GenericDocument> {
                name: collection_name,
                docs: vec![],
                schema: Map::new(),
            };

            // Iterate over the results of the cursor.
            while let Some(document) = cursor.try_next().await? {
                // println!("document: {:#?}", document);
                self.get_schema(document, &mut col.schema);
            }

            collection_map.push(col);
        }

        self.mongodb_collection = collection_map;
        Ok(&self.mongodb_collection)
    }

    pub fn cache(&self) -> Cache {
        Cache {
            collection_names: &self.collection_names,
            mongodb_collection: &self.mongodb_collection,
        }
    }

    pub fn cache_copy(&self) -> CacheCopy {
        CacheCopy {
            collection_names: self.collection_names.clone(),
            mongodb_collection: self.mongodb_collection.clone(),
        }
    }

    pub fn write_file(&self) -> io::Result<()> {
        fs::write(
            format!("tmp/{}_rs_v2.json", DBV1),
            serde_json::to_string_pretty(&self.mongodb_collection).unwrap(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::connect;
    use crate::db_common::DBV1;
    use crate::populate_v1::seed as seed_v1;

    #[tokio::test]
    async fn explorer_explore_db_schema() {
        seed_v1().await.expect("fail seed"); // FIXME testing data
        let client = connect().await.expect("error connect mongodb");
        let db = client.database(DBV1);
        let collection_names = db
            .list_collection_names(None)
            .await
            .expect("fail get collections");
        let collection_len = collection_names.len();

        let mut explorer = Explorer::new(None);

        let v = explorer
            .explore(&client.database(DBV1))
            .await
            .expect("fail explore");
        let len = v.len();

        let cache = explorer.cache();

        assert_eq!(len, collection_len);
        assert_eq!(cache.mongodb_collection.len(), collection_len);
        assert_eq!(cache.collection_names.len(), collection_len);
    }
}
