#[macro_use]
extern crate diesel;
use std::collections::BTreeMap;

use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{dev, Error, HttpMessage, HttpResponse};
use actix_web::{dev::ServiceRequest, middleware::Logger, App, HttpServer};
use actix_web_httpauth::{
    extractors::{
        bearer::{BearerAuth, Config},
        AuthenticationError,
    },
    middleware::HttpAuthentication,
};
use constants::{BURST_SIZE, PER_SECOND};
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

// Function to handle Unauthorized Error (401)
fn render_401<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, Error> {
    let error_response = json!({ "status": false, "message": "Unauthorized" });
    let new_response = HttpResponse::Unauthorized()
        .content_type("application/json")
        .body(error_response.to_string());
    // Convert the new response body into the appropriate type using `map_into_left_body`
    let (req, _res) = res.into_parts();
    let new_res = new_response.map_into_right_body::<B>();

    let new_service_response = dev::ServiceResponse::new(req, new_res);
    Ok(ErrorHandlerResponse::Response(new_service_response))
}

// Function to handle Unauthorized Error (400)
fn render_400<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, Error> {
    let error_response = json!({ "status": false, "message": "Endpoint not found" });
    let new_response = HttpResponse::NotFound()
        .content_type("application/json")
        .body(error_response.to_string());
    // Convert the new response body into the appropriate type using `map_into_left_body`
    let (req, _res) = res.into_parts();
    let new_res = new_response.map_into_right_body::<B>();

    let new_service_response = dev::ServiceResponse::new(req, new_res);
    Ok(ErrorHandlerResponse::Response(new_service_response))
}

// Function to handle JSON Parse Error
fn render_json_parse_error<B>(
    mut res: dev::ServiceResponse<B>,
) -> Result<ErrorHandlerResponse<B>, Error> {
    let error_response = json!({ "status": false, "message": "Failed to prase JSON" });
    let new_response = HttpResponse::UnprocessableEntity()
        .content_type("application/json")
        .body(error_response.to_string());
    // Convert the new response body into the appropriate type using `map_into_left_body`
    let (req, _res) = res.into_parts();
    let new_res = new_response.map_into_right_body::<B>();

    let new_service_response = dev::ServiceResponse::new(req, new_res);
    Ok(ErrorHandlerResponse::Response(new_service_response))
}

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
        let governor_conf = GovernorConfigBuilder::default()
            .per_second(PER_SECOND)
            .burst_size(BURST_SIZE)
            .finish()
            .unwrap();
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
            .wrap(Governor::new(&governor_conf))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                    ])
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .max_age(3600),
            )
            .wrap(
                ErrorHandlers::new()
                    .handler(actix_web::http::StatusCode::UNAUTHORIZED, render_401)
                    .handler(actix_web::http::StatusCode::NOT_FOUND, render_400)
                    .handler(
                        actix_web::http::StatusCode::BAD_REQUEST,
                        render_json_parse_error,
                    ), // Handle JSON parse error as Bad Request
            )
            .app_data(Data::new(pool.clone()))
            .with_json_spec_v3_at("/spec/561cc460-4fe6-4026-8470-6aae680086ca")
            //open routes
            .service(welcome)
            .service(web::scope("/admin").wrap(admin_auth))
            .service(web::scope("/organisations").wrap(auth))
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
