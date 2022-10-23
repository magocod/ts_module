use mgdb::populate_v1;
use mgdb::populate_v2;
use mgdb::populate_v3;
use mgdb::populate_v4;

#[tokio::main]
async fn main() {
    println!("init seed");

    populate_v1::seed().await.expect("fail populate_v1");
    populate_v2::seed().await.expect("fail populate_v2");
    populate_v3::seed().await.expect("fail populate_v3");
    populate_v4::seed().await.expect("fail populate_v4");

    println!("complete seed");
}
