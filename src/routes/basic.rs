use actix_web::{
    get,
    web::{self, ReqData},
    HttpResponse,
};
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{auth::generate_token, others::errors::CustomError};

#[derive(Serialize, Debug, Deserialize)]
pub struct Welcome {
    pub status: bool,
    pub message: String,
    pub token: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct User{
    pub status:bool,
    pub message:String,
    pub user_id:Uuid,
}

#[get("/")]
pub async fn welcome(
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
) -> Result<HttpResponse, CustomError> {
    let user_id = uuid::Uuid::new_v4();
    Ok(HttpResponse::Ok().json(Welcome {
        status: true,
        message: "Welcome to the application".to_owned(),
        token: generate_token(user_id)?,
    }))
}

#[get("/")]
pub async fn get_user_id(
    db: web::Data<Pool<ConnectionManager<PgConnection>>>,
    uid: Option<ReqData<Uuid>>,
) -> Result<HttpResponse, CustomError> {
    let u = uid.ok_or("failed to fetch userId from access token".to_owned())?;
    let user_id=u.into_inner();
    Ok(HttpResponse::Ok().json(User{status:true,message:"Token Authenticated successfully".to_owned(),user_id}))
}
