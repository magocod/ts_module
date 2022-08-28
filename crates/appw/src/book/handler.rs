use actix_web::{HttpResponse, Scope, web};

use crate::error::ServiceError;

/// base url handler
pub fn book_scope() -> Scope {
    web::scope("/books")
        .route("/", web::post().to(create))
        .route("/{id}", web::get().to(find_one))
        .route("/{id}", web::put().to(update))
        .route("/{id}", web::delete().to(delete))
        .route("/", web::get().to(find_all))
}

pub async fn find_all() -> Result<HttpResponse, ServiceError> {
    // TODO
    Ok(HttpResponse::Ok().json("all books"))
}

pub async fn find_one() -> Result<HttpResponse, ServiceError> {
    // TODO
    Ok(HttpResponse::Ok().json("find one book"))
}

pub async fn create() -> Result<HttpResponse, ServiceError> {
    // TODO
    Ok(HttpResponse::Ok().json("create book"))
}

pub async fn update() -> Result<HttpResponse, ServiceError> {
    // TODO
    Ok(HttpResponse::Ok().json("update book"))
}

pub async fn delete() -> Result<HttpResponse, ServiceError> {
    // TODO
    Ok(HttpResponse::Ok().json("delete book"))
}
