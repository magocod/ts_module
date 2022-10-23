use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::error::ServiceError;

use argon2::{self, Config};

pub const SECRET_KEY: &str = "secret";
pub const SALT: &[u8] = b"supersecuresalt";

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthPayload {
    // pub user: User,
    pub token: String,
}

// WARNING THIS IS ONLY FOR DEMO PLEASE DO MORE RESEARCH FOR PRODUCTION USE
pub fn hash_password(password: &str) -> Result<String, ServiceError> {
    let config = Config {
        secret: SECRET_KEY.as_bytes(),
        ..Default::default()
    };
    argon2::hash_encoded(password.as_bytes(), SALT, &config).map_err(|err| {
        dbg!(err);
        ServiceError::InternalServerError
    })
}

pub fn verify(hash: &str, password: &str) -> Result<bool, ServiceError> {
    argon2::verify_encoded_ext(hash, password.as_bytes(), SECRET_KEY.as_bytes(), &[]).map_err(
        |err| {
            dbg!(err);
            ServiceError::Unauthorized
        },
    )
}

pub async fn login() -> Result<HttpResponse, ServiceError> {
    // TODO
    Ok(HttpResponse::Ok().json({}))
}

pub async fn logout() -> Result<HttpResponse, ServiceError> {
    // TODO
    Ok(HttpResponse::Ok().json({}))
}

pub async fn get_auth_user() -> Result<HttpResponse, ServiceError> {
    // TODO
    Ok(HttpResponse::Ok().json({}))
}
