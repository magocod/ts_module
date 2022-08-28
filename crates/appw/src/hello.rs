use actix_web::{get, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

pub const APP_MESSAGE: &str = "Hello rust actix!";
pub const PUBLIC_ROUTES: [&str; 2] = ["/", "/public"];

#[derive(Debug, Serialize, Deserialize)]
pub struct MyObj {
    pub name: String,
}

#[get("/")]
pub async fn hello_server() -> impl Responder {
    HttpResponse::Ok().json(MyObj {
        name: String::from(APP_MESSAGE),
    })
}

#[get("/public_route")]
pub async fn public_route() -> HttpResponse {
    HttpResponse::Ok().json(MyObj { name: "public route".to_string() })
}

#[get("/auth_route")]
pub async fn auth_route() -> HttpResponse {
    HttpResponse::Ok().json(MyObj { name: "auth route".to_string() })
}

#[cfg(test)]
mod tests {
    // use std::assert_matches::assert_matches;
    use super::*;
    use actix_web::{test, App, http};

    #[actix_web::test]
    async fn return_json_content() {
        let app = test::init_service(App::new().service(hello_server)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        // let req_json = test::TestRequest::get().uri("/").to_request();

        let resp = test::call_service(&app, req).await;
        // let js: MyObj = test::call_and_read_body_json(&app, req_json).await;
        let status = resp.status();
        let js: MyObj = test::read_body_json( resp).await;

        // println!("{}", status);
        // println!("{:?}", js);

        assert!(status.is_success());
        assert_eq!(status, http::StatusCode::OK);
        assert_eq!(js.name, APP_MESSAGE);
        // assert_matches!(js, [My])
    }
}
