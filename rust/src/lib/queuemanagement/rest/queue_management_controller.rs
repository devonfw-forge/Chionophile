use actix_web::{web};
use super::queue_management_service::*;

pub fn queue_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/queue")
            .route(web::post().to(save_queue))
    ).service(
        web::resource("/queue/search")
            .route(web::post().to(find_queues))
    ).service(
        web::resource("/queue/{id}")
            .route(web::get().to(get_queue))
            .route(web::delete().to(delete_queue))
    );
}