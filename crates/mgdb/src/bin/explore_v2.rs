use mgdb::db::connect;
use mgdb::db_common::DBV1;
use mgdb::db_v2::Explorer;
use pluralizer;

#[tokio::main]
async fn main() {
    pluralizer::initialize();

    let client = connect().await.expect("error connect mongodb");
    let mut explorer = Explorer::new(None);
    explorer
        .explore(&client.database(DBV1))
        .await
        .expect("explore failed");

    let c = explorer.cache();
    println!("{:#?}", c);

    explorer.write_file().expect("fail generate file");
}
