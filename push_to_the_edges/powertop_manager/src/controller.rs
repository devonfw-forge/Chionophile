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
    let mut command = Command::new("powertop");
    let time_command = format!("--time={}",params.time);
    let csv_command = format!("--csv={}",params.csv);
    command.args([time_command, csv_command]);
    // println!("{:?}", command);
    let program = command.spawn();
    if program.is_err() {
        Ok(HttpResponse::InternalServerError().body("Could not start powertop"))
    } else {
        Ok(HttpResponse::Ok().body("Powertop started"))
    }

}