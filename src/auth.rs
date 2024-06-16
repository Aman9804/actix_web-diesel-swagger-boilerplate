use std::{collections::HashMap, future::Future, pin::Pin};

use actix_web::{dev::Payload, FromRequest, HttpRequest};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use paperclip::actix::Apiv2Security;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::others::errors::CustomError;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct UserClaims {
    sub: String,
    company: String,
    user_id: Uuid,
    exp: usize,
}
#[derive(Apiv2Security)]
#[openapi(
    apiKey,
    in = "header",
    name = "Authorization",
    description = "Use format 'Bearer TOKEN'"
)]
pub struct AccessToken {
    pub token: String,
}

impl FromRequest for AccessToken {
    type Error = CustomError;
    type Future = Pin<Box<dyn Future<Output = Result<AccessToken, CustomError>>>>;

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        let token = req
            .headers()
            .get("Authorization")
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();

        Box::pin(async move {
            let claim_future = get_token(token.clone());
            claim_future
        })
    }
}

pub fn get_token(token: String) -> Result<AccessToken, CustomError> {
    Ok(AccessToken { token })
}
pub fn validate_token(token: &str) -> Result<Uuid, CustomError> {
    //return user_id
    let key = std::env::var("SECRET")?;
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<UserClaims>(
        &token,
        &DecodingKey::from_secret(key.as_bytes()),
        &validation,
    )?;
    Ok(token_data.claims.user_id)
}

pub fn generate_user_token(user_id: Uuid) -> Result<String, CustomError> {
    let key = std::env::var("SECRET")?;
    let my_claims = UserClaims {
        sub: "aman@xynes.com".to_owned(),
        company: "Xynes".to_owned(),
        exp: 10000000000,
        user_id,
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key.as_bytes()),
    )?;
    Ok(token)
}

#[derive(Deserialize)]
pub struct PermissionCheckResponse {
    status: bool,
}

pub async fn has_permission(
    access_token: AccessToken,
    permission_type: String,
    entity: String,
    data: String,
) -> Result<bool, CustomError> {
    let mut map = HashMap::new();
    map.insert("token", access_token.token);
    map.insert("permission_type", permission_type);
    map.insert("submodule", entity);
    map.insert("data", data);
    map.insert("module", "admin-panel".to_string());
    let client = reqwest::Client::new();
    let res = client
        .post(format!("http://admin.xynes.com/api/check-permission"))
        .json(&map)
        .send()
        .await?;
    let res = res.json::<PermissionCheckResponse>().await?;
    Ok(res.status)
}

#[derive(Debug, Serialize, Deserialize)]
struct AdminClaims {
    sub: String,
    company: String,
    admin_id: uuid::Uuid,
    exp: usize,
}
pub fn generate_admin_token(admin_id: Uuid) -> Result<String, CustomError> {
    let key = std::env::var("ADMINSECRET")?;
    let my_claims = AdminClaims {
        sub: "aman@xynes.com".to_owned(),
        company: "Xynes".to_owned(),
        exp: (Utc::now().naive_utc().timestamp() + 3600) as usize,
        admin_id,
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(key.as_bytes()),
    )?;
    Ok(token)
}
pub fn validate_admin_token(token: &str) -> Result<Uuid, CustomError> {
    //return user_id
    let key = std::env::var("ADMINSECRET")?;
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<AdminClaims>(
        &token,
        &DecodingKey::from_secret(key.as_bytes()),
        &validation,
    )?;
    Ok(token_data.claims.admin_id)
}
