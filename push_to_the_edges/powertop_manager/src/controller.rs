use std::process::Command;
use actix_web::{Error, HttpResponse, web};
use crate::powertop_params::PowerTopParams;

pub fn scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/start")
            .route(web::post().to(powertop_start))
    );
}

async fn powertop_start(
    params: web::Json<PowerTopParams>
) -> Result<HttpResponse, Error>{
    let program = Command::new("powertop")
        .args(["--workload", &params.workload, "--csv", &params.csv])
        .spawn();
    if program.is_err() {
        Ok(HttpResponse::InternalServerError().body("Could not start powertop"))
    } else {
        Ok(HttpResponse::Ok().body("Powertop started"))
    }

}