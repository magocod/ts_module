use actix_cors::Cors;
use actix_web::{dev::ServiceRequest, get, App, Error, HttpResponse, HttpServer, web};
use actix_web_httpauth::{extractors::bearer::BearerAuth, middleware::HttpAuthentication};

use appw::{MyObj};
use appw::auth;

async fn ok_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    eprintln!("{}", req.path());

    eprintln!("{:?}", credentials);
    println!("token: {}", credentials.token());

    // if req.path().starts_with("/api/v1/auth") {
    //     return Ok(req)
    // }

    Ok(req)
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().json(MyObj { name: "public base route".to_string() })
}

#[get("/public")]
async fn public() -> HttpResponse {
    HttpResponse::Ok().json(MyObj { name: "public route".to_string() })
}

#[get("/auth_user")]
async fn auth_user() -> HttpResponse {
    HttpResponse::Ok().json(MyObj { name: "auth route".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // ensure the CORS middleware is wrapped around the httpauth middleware so it is able to
            // add headers to error responses
            .wrap(Cors::permissive())
            .service(index)
            .service(public)
            .service(
                web::scope("/auth")
                    .route("/login", web::post().to(auth::login))
                    .route("/logout", web::post().to(auth::logout))
                    .route("/current", web::post().to(auth::get_auth_user))
            )
            .service(
                web::scope("/api")
                    .wrap(HttpAuthentication::bearer(ok_validator))
                    .service(auth_user)
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
