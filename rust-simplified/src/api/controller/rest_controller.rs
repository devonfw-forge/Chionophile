use actix_web::{web, Scope};

pub trait RestController {
    fn scope(cfg: &mut web::ServiceConfig);

    fn get_scope(base_url: &String) -> Scope;
}
