use crate::domain::config::{app_config::read_config, service_config::ServiceConfig};
use crate::domain::{config::dbtypes_config::*, state::app_state::AppState};
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use num_cpus;

#[macro_use]
extern crate custom_derive;

extern crate openssl;
#[macro_use]
extern crate diesel;

mod api;
mod domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    openssl_probe::init_ssl_cert_env_vars();
    dotenv::dotenv().ok();
    let app_config = read_config();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let manager = ConnectionManager::<DbConn>::new(app_config.database_url);
    let pool = r2d2::Pool::builder().build(manager).expect("Failed to create pool");

    HttpServer::new(move || {
        let service_config = ServiceConfig::new(&app_config.base_rest_url);
        let cors = Cors::permissive();
        App::new()
            .app_data(web::Data::new(AppState::new(pool.clone())))
            //.wrap(middleware::Logger::default())
            .wrap(cors)
            .service(service_config.visitor_scope)
            .service(service_config.queue_scope)
            .service(service_config.accesscode_scope)
    })
    .bind(app_config.bind_url)?
    .workers(num_cpus::get())
    .run()
    .await
}
