#[macro_use]
extern crate diesel;
use actix_cors::Cors;

use actix_web::{dev::ServiceRequest, middleware::Logger, App, HttpServer};
use actix_web::{HttpMessage};
use actix_web_httpauth::{
    extractors::{
        bearer::{BearerAuth, Config},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use routes::basic::{get_user_id, welcome};
mod auth;
mod models;
mod others;
mod routes;
mod schema;

use paperclip::actix::{
    // If you prefer the macro syntax for defining routes, import the paperclip macros
    // get, post, put, delete
    // use this instead of actix_web::web
    web::{self, Data},

    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt,
};
use routes::users::{get_all_users, get_users, update_users, delete_users, create_users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let manager = ConnectionManager::<PgConnection>::new(database_url);
    // let pool = r2d2::Pool::builder()
    //     .build(manager)
    //     .expect("Failed to create pool.");

    // create a new connection pool with the default config
    let config: AsyncDieselConnectionManager<diesel_async::AsyncPgConnection> =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config).build().unwrap();

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);

        App::new()
            .wrap(Cors::default()
              .allow_any_origin()
              .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600))
            .wrap_api()
            .wrap(Logger::default())
            // .wrap(auth)
            .app_data(Data::new(pool.clone()))
            .service(welcome)
            
            .service(get_all_users)
            .service(get_users)
            .service(update_users)
            .service(delete_users)
            .service(create_users)
            
            .with_json_spec_at("/api/spec/v2") //open routes
            .service(
                web::scope("").wrap(auth).service(get_user_id) //authenticated routes
            )
            .build()
    })
    .bind(("0.0.0.0", 5003))?
    .run()
    .await
}

async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let db = req
        .app_data::<Data<Pool<diesel_async::AsyncPgConnection>>>()
        .expect("Failed to extract DatabaseConnection from ServiceRequest")
        .get_ref()
        .clone();
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    match auth::validate_token(credentials.token()) {
        Ok(user_id) => {
            req.extensions_mut().insert(user_id);
            Ok(req)
        }
        Err(_) => Err((AuthenticationError::from(config).into(), req)),
    }
}

// fn get_file(url: &str, save_name: &str) -> String {
//     let resp = reqwest::blocking::get(url).expect("request failed");
//     let body = resp.text().expect("body invalid");
//     let mut out = File::create(save_name).expect("failed to create file");
//     io::copy(&mut body.as_bytes(), &mut out).expect("failed to copy content");
//     save_name.to_owned()
// }
