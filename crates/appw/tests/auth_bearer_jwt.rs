use actix_web::{http, test, App, web};
use actix_web_httpauth::{middleware::HttpAuthentication};

use appw::jwt::{ok_validator, generate_token};
use appw::hello::{auth_route, MyObj};
use appw::user::models::User;

mod common;

// TODO complete tests

#[actix_web::test]
async fn successful_authentication() {
    let app = test::init_service(App::new().service(
        web::scope("/api")
            .wrap(HttpAuthentication::bearer(ok_validator))
            .service(auth_route)
    )).await;

    let tk = generate_token(&User::factory()).expect("error generate token");

    let req = test::TestRequest::get()
        .uri("/api/auth_route")
        .insert_header(common::set_auth_header(tk))
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::OK);

    let js: MyObj = test::read_body_json( resp).await;
    println!("{:?}", js);
}

#[actix_web::test]
async fn no_authentication_header() {
    let app = test::init_service(App::new().service(
        web::scope("/api")
            .wrap(HttpAuthentication::bearer(ok_validator))
            .service(auth_route)
    )).await;

    let req = test::TestRequest::get().uri("/api/auth_route").to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::UNAUTHORIZED);
}

// #[actix_web::test]
// async fn incomplete_authentication_header() {
//     let app = test::init_service(App::new().service(scope)).await;
//
//     let req = test::TestRequest::get()
//         .uri("/api/jwt")
//         .insert_header((http::header::AUTHORIZATION, "bearer"))
//         .to_request();
//
//     let resp = test::call_service(&app, req).await;
//
//     println!("{}, {:?}", resp.status(), resp.response().body());
//     assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
// }
//
// #[actix_web::test]
// async fn invalid_token() {
//     let app = test::init_service(App::new().service(scope)).await;
//
//     let req = test::TestRequest::get()
//         .uri("/api/jwt")
//         .insert_header((
//             http::header::AUTHORIZATION,
//             "bearer invalidadtk",
//         ))
//         .to_request();
//
//     let resp = test::call_service(&app, req).await;
//
//     println!("{}, {:?}", resp.status(), resp.response().body());
//     assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
// }
