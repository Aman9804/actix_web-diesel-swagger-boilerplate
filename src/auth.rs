
use std::{pin::Pin, future::Future};

use actix_web::{FromRequest, HttpRequest, dev::Payload};
use paperclip::actix::{ Apiv2Security};
use serde::{Serialize, Deserialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey};
use uuid::Uuid;

use crate::{others::errors::{CustomError}};

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    user_id:Uuid,
    exp: usize,
}
#[derive(Apiv2Security)]
#[openapi(
  apiKey,
  in = "header",
  name = "Authorization",
  description = "Use format 'Bearer TOKEN'"
)]
pub struct AccessToken{
    pub token:String
}

impl FromRequest for AccessToken{
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<AccessToken, CustomError>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let token =req.headers().get("Authorization").unwrap()
            .to_str()
            .unwrap()
            .to_owned();
 
        Box::pin(async move {
                let claim_future = get_token(token.clone());
                claim_future
        })
    }
}

pub fn get_token(token: String) -> Result<AccessToken, CustomError>
{
    Ok(AccessToken{token})
}
pub fn validate_token(token: &str) -> Result<Uuid, CustomError> {
    //return user_id
    let key=std::env::var("SECRET")?;
    let validation = Validation::new(Algorithm::HS256);
    let token_data=decode::<Claims>(&token, &DecodingKey::from_secret(key.as_bytes()), &validation)?; 
    Ok(token_data.claims.user_id)
}

pub fn generate_token(user_id:Uuid)->Result<String,CustomError>{
    let key = std::env::var("SECRET")?;
    let my_claims = Claims {
        sub: "aman@xynes.com".to_owned(),
        company: "Xynes".to_owned(),
        exp: 10000000000,
        user_id
    };
    let token=encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key.as_bytes())) ?;
    Ok(token)

}