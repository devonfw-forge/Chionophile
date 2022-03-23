use actix_web::web;
use crate::lib::visitormanagement::rest::visitor_management_service::*;

pub fn visitor_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/visitor/search")
            .route(web::post().to(find_visitors))
    ).service(
        web::resource("/visitor")
            .route(web::post().to(save_visitor))
    ).service(
        web::resource("/visitor/{id}")
            .route(web::get().to(get_visitor))
            .route(web::delete().to(delete_visitor))
    );
}
