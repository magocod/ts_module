use mgdb::populate_v1;

#[tokio::main]
async fn main() {
    println!("init seed");

    populate_v1::seed().await.expect("fail populate_v1");

    println!("complete seed");
}
