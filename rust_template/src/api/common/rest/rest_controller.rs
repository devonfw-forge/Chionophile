use actix_web::web;

pub trait RestController {
    fn scope(cfg: &mut web::ServiceConfig);
}
