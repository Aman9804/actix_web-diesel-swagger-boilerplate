/* This file is generated and managed by xynes */


use diesel_async::pooled_connection::deadpool::Pool;
use serde::{Deserialize, Serialize};
use paperclip::actix::{
    Apiv2Schema, api_v2_operation,
    web::{self,Json},
    get,put,post,delete
};

use crate::{others::errors::CustomError};
use crate::models::users::*;



#[derive(Serialize, Debug, Deserialize,Apiv2Schema)]
pub struct Output {
    pub status: bool,
}



#[derive(Serialize, Debug, Deserialize,Apiv2Schema)]
pub struct Page {
    pub page: i64,
    pub page_size: i64,
}
        
///This api will fetch users details in paginated format
#[api_v2_operation]
#[get("/users")]
pub async fn get_all_users(
    db: web::Data<Pool<diesel_async::AsyncPgConnection>>,
    inp: web::Query<Page>,
) -> Result<Json<PaginationResult<User>>, CustomError> {
    let mut db=db.get().await?;
    let conn=db.as_mut();
    let res=User::paginate(conn, inp.page,inp.page_size).await?;
    Ok(Json(res))
}
        
///This api will fetch User details based on id provided
#[api_v2_operation]
#[get("/users/{users_id}")]
pub async fn get_users(
    db: web::Data<Pool<diesel_async::AsyncPgConnection>>,
    path: web::Path<uuid::Uuid>,
) -> Result<Json<User>, CustomError> {
    let mut db=db.get().await?;
    let conn=db.as_mut();
    let users_id=path.into_inner();
    let res=User::read(conn, users_id).await?;
    Ok(Json(res))
}
        
///This api will update User details based on id provided
#[api_v2_operation]
#[put("/users/{users_id}")]
pub async fn update_users(
    db: web::Data<Pool<diesel_async::AsyncPgConnection>>,
    path: web::Path<uuid::Uuid>,
    inp:  Json<UpdateUser>,
) -> Result<Json<User>, CustomError> {
    let mut db=db.get().await?;
    let conn=db.as_mut();
    let users_id=path.into_inner();
    let res=User::update(conn, users_id, &inp.0).await?;
    Ok(Json(res))
}
        
///This api will create one User 
#[api_v2_operation]
#[post("/users")]
pub async fn create_users(
    db: web::Data<Pool<diesel_async::AsyncPgConnection>>,
    inp:  Json<CreateUser>,
) -> Result<Json<User>, CustomError> {
    let mut db=db.get().await?;
    let conn=db.as_mut();
    let res=User::create(conn, &inp.0).await?;
    Ok(Json(res))
}
        
///This api will delete User based on id
#[api_v2_operation]
#[delete("/users/{users_id}")]
pub async fn delete_users(
    db: web::Data<Pool<diesel_async::AsyncPgConnection>>,
    path: web::Path<uuid::Uuid>,
) -> Result<Json<Output>, CustomError> {
    let mut db=db.get().await?;
    let conn=db.as_mut();
    let users_id=path.into_inner();
    let count=User::delete(conn, users_id).await?;
    let res=if count==0 {Output{status:false}} else {Output{status:true}};
    Ok(Json(res))
}
        