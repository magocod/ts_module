use actix_web::{App, HttpServer};

use appw::hello::hello_server;
use appw::book::handler::book_scope;
use appw::user::handler::user_scope;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello_server)
            .service(book_scope())
            .service(user_scope())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
