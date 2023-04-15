
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