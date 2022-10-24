use serde::{Deserialize, Serialize};

use jsonwebtoken::errors::Result as ResultJwt;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation,
};

use actix_web::{dev::ServiceRequest, Error as ActixError};
use actix_web_httpauth::extractors::bearer::BearerAuth;

use crate::auth::SECRET_KEY;
use crate::error::ServiceError;
use crate::user::models::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    id: String,
    exp: usize,
}

pub fn generate_token(user: &User) -> ResultJwt<String> {
    let my_claims = Claims {
        id: user.email.clone(),
        exp: 10000000000,
    };

    let header = Header {
        kid: Some("signing_key".to_owned()),
        ..Default::default()
    };

    let token = encode(
        &header,
        &my_claims,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    )?;
    // println!("{:?}", token);

    Ok(token)
}

pub fn decode_token(token: &String) -> ResultJwt<TokenData<Claims>> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;

    // println!("{:?}", token_data.claims.sub);
    // println!("{:?}", token_data.claims);
    // println!("{:?}", token_data.header);

    // let user: User = serde_json::from_str(token_data.claims.id.as_str())?;
    Ok(token_data)
}

pub async fn ok_validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    println!("{}", req.path());

    let tk = credentials.token().to_string();

    println!("{:?}", credentials);
    println!("token: {}", tk);

    match decode_token(&tk) {
        Ok(v) => {
            println!("{:#?}", v.claims);
            // TODO check user
            Ok(req)
        }
        Err(_e) => Err((ServiceError::Unauthorized.into(), req)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_token_from_user() {
        let test_user = User::factory();
        let tk = generate_token(&test_user).expect("generate_token error");

        println!("{:?}", test_user);
        println!("{}", tk);
        // assert_eq!(test_user.email.type_id(), );
    }
}
