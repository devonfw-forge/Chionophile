use actix_cors::Cors;
use actix_web::{App, HttpServer, web};
use crate::config::read_config;

mod controller;
mod powertop_params;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let app_config = read_config();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .wrap(cors)
            .service(web::scope("powertop").configure(controller::scope))
    })
        .bind(app_config.bind_url)?
        .workers(1)
        .run()
        .await
}
