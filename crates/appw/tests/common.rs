use actix_web::http;
use actix_web::http::header::HeaderName;

#[allow(dead_code)]
pub fn setup() -> () {
    ()
}

pub fn set_auth_header(tk: String) -> (HeaderName, String) {
    (http::header::AUTHORIZATION, format!("Bearer {}", tk))
}
