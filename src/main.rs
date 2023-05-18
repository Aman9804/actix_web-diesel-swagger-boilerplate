#[macro_use]
extern crate diesel;
use std::collections::BTreeMap;

use actix_cors::Cors;

use actix_web::HttpMessage;
use actix_web::{dev::ServiceRequest, middleware::Logger, App, HttpServer};
use actix_web_httpauth::{
    extractors::{
        bearer::{BearerAuth, Config},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use paperclip::v2::models::{DefaultApiRaw, Info};
use routes::basic::welcome;
mod auth;
mod constants;
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

use serde_json::{json, Value};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create a new connection pool with the default config
    let config: AsyncDieselConnectionManager<diesel_async::AsyncPgConnection> =
        AsyncDieselConnectionManager::<diesel_async::AsyncPgConnection>::new(database_url);
    let pool = Pool::builder(config).build().unwrap();

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);

        let admin_auth = HttpAuthentication::bearer(validator_admin);

        let mut spec = DefaultApiRaw::default();
        spec.base_path = Some("https://bookings.xynes.com/api".into());
        spec.info = Info {
            version: "0.1".into(),
            title: "Booking System".into(),
            ..Default::default()
        };
        let mut ext: BTreeMap<String, Value> = BTreeMap::new();
        ext.insert(
            "x-samples-languages".into(),
            json!(["curl", "python", "go"]),
        );
        spec.extensions = ext;

        App::new()
            .wrap(Logger::default())
            .wrap_api_with_spec(spec)
            // .wrap(auth)
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .app_data(Data::new(pool.clone()))
            .with_json_spec_v3_at("/spec/561cc460-4fe6-4026-8470-6aae680086ca")
            //open routes
            .service(welcome)
            .service(
                web::scope("/admin")
                    .wrap(admin_auth)
                    ,
            )
            .service(
                web::scope("/organisations")
                    .wrap(auth)
                    
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

async fn validator_admin(
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

    match auth::validate_admin_token(credentials.token()) {
        Ok(user_id) => {
            req.extensions_mut().insert(user_id);
            Ok(req)
        }
        Err(_) => Err((AuthenticationError::from(config).into(), req)),
    }
}
