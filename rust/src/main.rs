use actix_web::{middleware, web, App, HttpServer};
use diesel::{r2d2::{self, ConnectionManager}};
use actix_cors::Cors;
use crate::lib::general::config::config::read_config;
use crate::lib::general::config::db_config::{DbConn, DbError};
use crate::lib::queuemanagement::rest::queue_management_controller::queue_scope;
use crate::lib::visitormanagement::rest::visitor_management_controller::visitor_scope;
use crate::lib::accesscodemanagement::rest::accesscode_management_controller::access_code_scope;

#[macro_use]
extern crate diesel;
extern crate core;

mod schema;
mod lib;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let app_config = read_config();

    //Pool for database connections
    let manager = ConnectionManager::<DbConn>::new(app_config.database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    let visitor_url = app_config.base_rest_url.to_owned() + "visitormanagement/v1";
    let queue_url = app_config.base_rest_url.to_owned() + "queuemanagement/v1";
    let accesscode_url = app_config.base_rest_url.to_owned() + "accesscodemanagement/v1";

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(web::scope(&visitor_url).configure(visitor_scope))
            .service(web::scope(&queue_url).configure(queue_scope))
            .service(web::scope(&accesscode_url).configure(access_code_scope))
    })
    .bind(app_config.bind_url)?
    .run()
    .await
}
