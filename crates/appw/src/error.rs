use actix_web::{
    error::ResponseError,
    HttpResponse,
    // Error as ActixError
};
use jsonwebtoken::errors::{Error as ErrorJwt};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum ServiceError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,

    #[display(fmt = "BadRequest: {}", _0)]
    BadRequest(String),

    #[display(fmt = "NotFound: {}", _0)]
    NotFound(String),

    #[display(fmt = "Unauthorized")]
    Unauthorized,
}

// impl ResponseError trait allows to convert our errors into http responses with appropriate data
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error, Please try later")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::NotFound(ref message) => HttpResponse::NotFound().json(message),
        }
    }
}

// impl From<Example> for ServiceError {
//     fn from(_: Example) -> ServiceError {
//         ServiceError::BadRequest("error".into())
//     }
// }

impl From<ErrorJwt> for ServiceError {
    fn from(_v: ErrorJwt) -> ServiceError {
        // match v.kind() {
        //     ErrorKind::InvalidToken => {}
        //     ErrorKind::InvalidSignature => {}
        //     ErrorKind::InvalidEcdsaKey => {}
        //     ErrorKind::InvalidRsaKey(_) => {}
        //     ErrorKind::RsaFailedSigning => {}
        //     ErrorKind::InvalidAlgorithmName => {}
        //     ErrorKind::InvalidKeyFormat => {}
        //     ErrorKind::MissingRequiredClaim(_) => {}
        //     ErrorKind::ExpiredSignature => {}
        //     ErrorKind::InvalidIssuer => {}
        //     ErrorKind::InvalidAudience => {}
        //     ErrorKind::InvalidSubject => {}
        //     ErrorKind::ImmatureSignature => {}
        //     ErrorKind::InvalidAlgorithm => {}
        //     ErrorKind::MissingAlgorithm => {}
        //     ErrorKind::Base64(_) => {}
        //     ErrorKind::Json(_) => {}
        //     ErrorKind::Utf8(_) => {}
        //     ErrorKind::Crypto(_) => {}
        // }
        ServiceError::BadRequest("error jwt".into())
    }
}

// impl From<ActixError> for ServiceError {
//     fn from(_v: ActixError) -> ServiceError {
//         ServiceError::BadRequest("error".into())
//     }
// }
