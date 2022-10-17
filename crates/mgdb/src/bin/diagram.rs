use mgdb::db::explore;
use std::fs;
use pluralizer;

#[tokio::main]
async fn main() {
    pluralizer::initialize();

    let v = explore().await.expect("explore failed");
    println!("v: {:#?}", v);

    fs::write("tmp/rs.json", serde_json::to_string_pretty(&v).unwrap())
        .expect("Unable to write file");
}
