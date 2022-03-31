use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web};
use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use crate::core::general::config::app_config::read_config;
use crate::core::general::config::dbtypes_config::DbConn;
use crate::core::general::config::service_config::ServiceConfig;
use crate::core::general::state::app_state::AppState;

//#[macro_use]
//extern crate lazy_static;
#[macro_use]
extern crate diesel;

mod core;
mod api;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    /*
        Logger and app configuration based on environment variables and config files.
        The method read_config reads the environment variables related to the API and Database
        IPs and URLs.
        init_from_env initializes the logger that will show the incoming requests when the
        API is running.
     */
    dotenv::dotenv().ok();
    let app_config = read_config();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    /*
        This established a pool of connections with the specified database.
        The type DbConn is an alias for the selected database connection in the config file
        located at core/general/config/dbtypes_config.rs. In this template, it translates to
        PgConnection, because we are using PostgreSQL.
    */
    let manager = ConnectionManager::<DbConn>::new(app_config.database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    /*
        Initialize the server.
        - The application will have an AppState struct where we will store all the data that is needed
          between threads and/or multiple endpoints. The only required field is the database pool, but
          it can be used for other purposes such as mutexes.
        - Instantiate the service config. This struct will store every entity scope in the app.
        - Initialize the CORS configuration. In this case is permissive for testing purposes,
          but it should be changed in production for security reasons. It's defined inside
          the HttpServer because it doesn't implement the Clone trait.
    */
    HttpServer::new(move || {
        let service_config = ServiceConfig::new(&app_config.base_rest_url);
        let cors = Cors::permissive();
        App::new()
            .app_data(web::Data::new(AppState::new(pool.clone())))
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .service(service_config.user_scope)
    })
        .bind(app_config.bind_url)?
        .run()
        .await
}
