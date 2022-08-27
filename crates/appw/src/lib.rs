use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub mod auth;
pub mod error;

pub const APP_MESSAGE: &str = "Hello rust actix!";
pub const PUBLIC_ROUTES: [&str; 2] = ["/", "/public"];

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
    pub name: String,
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().json(MyObj {
        name: String::from(APP_MESSAGE),
    })
}

#[cfg(test)]
mod tests {
    // use std::assert_matches::assert_matches;
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn return_json_content() {
        let app = test::init_service(App::new().service(hello)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let req_json = test::TestRequest::get().uri("/").to_request();

        let resp = test::call_service(&app, req).await;
        let js: MyObj = test::call_and_read_body_json(&app, req_json).await;

        // println!("{}, {:?}", resp.status(), resp.response().body());
        // println!("{:?}", js);

        assert!(resp.status().is_success());
        assert_eq!(js.name, APP_MESSAGE);
        // assert_matches!(js, [My])
    }
}
