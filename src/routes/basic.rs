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

use crate::{auth::{ AccessToken}, others::errors::CustomError};

#[derive(Serialize, Debug, Deserialize,Apiv2Schema)]
pub struct Welcome {
    pub status: bool,
    pub message: String,
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

#[api_v2_operation(
    summary = "This api will welcome you",
    tags("Open Routes")
)]
#[get("/")]
pub async fn welcome(
    db: web::Data<Pool<diesel_async::AsyncPgConnection>>
) -> Result<Json<Welcome>, CustomError> {
    let user_id = uuid::Uuid::new_v4();
    Ok(Json(Welcome {
        status: true,
        message: "Welcome to the Booking system API".to_owned(),
    }))
}
