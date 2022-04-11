use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web};
use diesel::r2d2;
use num_cpus;
use diesel::r2d2::ConnectionManager;
use crate::core::general::config::app_config::read_config;
use crate::core::general::config::dbtypes_config::DbConn;
use crate::core::general::config::service_config::ServiceConfig;
use crate::core::general::state::app_state::AppState;

#[macro_use]
extern crate diesel;

mod core;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
    let app_config = read_config();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let manager = ConnectionManager::<DbConn>::new(app_config.database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

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
