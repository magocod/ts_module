use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use actix_web_httpauth::{middleware::HttpAuthentication};

use appw::hello::{hello_server, public_route, auth_route};
use appw::auth;
use appw::jwt::ok_validator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // ensure the CORS middleware is wrapped around the httpauth middleware so it is able to
            // add headers to error responses
            .wrap(Cors::permissive())
            .service(hello_server)
            .service(public_route)
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(auth::login))
                    .route("/logout", web::post().to(auth::logout))
                    .route("/current", web::get().to(auth::get_auth_user))
            )
            .service(
                web::scope("/api")
                    .wrap(HttpAuthentication::bearer(ok_validator))
                    .service(auth_route)
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
