use actix_web::{web, HttpResponse, Scope};

use crate::error::ServiceError;

/// base url handler
pub fn user_scope() -> Scope {
    web::scope("/users")
        .route("/", web::post().to(create))
        .route("/{id}", web::get().to(find_one))
        .route("/{id}", web::put().to(update))
        .route("/{id}", web::delete().to(delete))
        .route("/", web::get().to(find_all))
}

pub async fn find_all() -> Result<HttpResponse, ServiceError> {
    // TODO
    Ok(HttpResponse::Ok().json("all users"))
}

pub async fn find_one() -> Result<HttpResponse, ServiceError> {
    // TODO
    Ok(HttpResponse::Ok().json("find one user"))
}

pub async fn create() -> Result<HttpResponse, ServiceError> {
    // TODO
    Ok(HttpResponse::Ok().json("create user"))
}

pub async fn update() -> Result<HttpResponse, ServiceError> {
    // TODO
    Ok(HttpResponse::Ok().json("update user"))
}

pub async fn delete() -> Result<HttpResponse, ServiceError> {
    // TODO
    Ok(HttpResponse::Ok().json("delete user"))
}
