use actix_web::{
    HttpResponse,
};

use diesel_async::pooled_connection::deadpool::Pool;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use paperclip::actix::{
    // extension trait for actix_web::App and proc-macro attributes
     Apiv2Schema, api_v2_operation,
    // If you prefer the macro syntax for defining routes, import the paperclip macros
    // get, post, put, delete
    // use this instead of actix_web::web
    web::{self,ReqData,Json},
    get
};

use crate::{auth::{generate_token, AccessToken}, others::errors::CustomError};

#[derive(Serialize, Debug, Deserialize,Apiv2Schema)]
pub struct Welcome {
    pub status: bool,
    pub message: String,
    pub token: String,
}
// impl Responder for Welcome {
//     type Body = BoxBody;

//     fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
//         let body = serde_json::to_string(&self).unwrap();

//         // Create response and set content type
//         HttpResponse::Ok()
//             .content_type(ContentType::json())
//             .body(body)
//     }
// }

#[derive(Serialize, Debug, Deserialize,Apiv2Schema)]
pub struct User{
    pub status:bool,
    pub message:String,
    pub user_id:Uuid,
}

///This api will welcome you and give you an access token to be used in other api calls
#[api_v2_operation]
#[get("/")]
pub async fn welcome(
    db: web::Data<Pool<diesel_async::AsyncPgConnection>>
) -> Result<Json<Welcome>, CustomError> {
    let user_id = uuid::Uuid::new_v4();
    Ok(Json(Welcome {
        status: true,
        message: "Welcome to the application".to_owned(),
        token: generate_token(user_id)?,
    }))
}




#[api_v2_operation]
#[get("/check-token")]
pub async fn get_user_id(
    _a:AccessToken,
    db:web::Data<Pool<diesel_async::AsyncPgConnection>>,
    uid: Option<ReqData<Uuid>>,
) -> Result<HttpResponse, CustomError> {
    let u = uid.ok_or("failed to fetch userId from access token".to_owned())?;
    let user_id=u.into_inner();
    Ok(HttpResponse::Ok().json(User{status:true,message:"Token Authenticated successfully".to_owned(),user_id}))
}
