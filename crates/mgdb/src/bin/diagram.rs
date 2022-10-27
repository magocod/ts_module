use mgdb::db::{connect, explore};
use mgdb::db_common::DBV1;
use pluralizer;
use std::fs;

#[tokio::main]
async fn main() {
    pluralizer::initialize();

    let client = connect().await.expect("error connect mongodb");
    let v = explore(&client, DBV1).await.expect("explore failed");
    println!("v: {:#?}", v);

    fs::write(
        format!("tmp/{}_rs.json", DBV1),
        serde_json::to_string_pretty(&v).unwrap(),
    )
    .expect("Unable to write file");
}
