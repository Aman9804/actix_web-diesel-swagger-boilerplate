#[macro_use]
extern crate diesel;


use actix_web::{HttpServer, App, middleware::Logger, web::{Data, self}, dev::ServiceRequest};
use actix_web_httpauth::{middleware::HttpAuthentication, extractors::{AuthenticationError, bearer::{BearerAuth, Config}}};
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;
use actix_web::HttpMessage;
use routes::basic::{get_user_id, welcome};
mod auth;
mod others;
mod routes;
#[actix_web::main]
async fn main()-> std::io::Result<()> {
    dotenv::dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);

        App::new()
            .wrap(Logger::default())
            // .wrap(auth)
            .app_data(Data::new(pool.clone()))
            .service(welcome) //open routes
            .service(
                web::scope("").wrap(auth).service(get_user_id), //authenticated routes
            )
    })
    .bind(("0.0.0.0", 5002))?
    .run()
    .await
}


async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let db = req
        .app_data::<actix_web::web::Data<Pool<ConnectionManager<PgConnection>>>>()
        .expect("Failed to extract DatabaseConnection from ServiceRequest")
        .get_ref()
        .clone();
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    match auth::validate_token( credentials.token()) {
        Ok(user_id) => {
            req.extensions_mut().insert(user_id);
            Ok(req)
        }
        Err(_) => Err((AuthenticationError::from(config).into(), req)),
    }
}
