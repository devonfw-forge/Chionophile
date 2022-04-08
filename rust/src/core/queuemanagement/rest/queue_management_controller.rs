use actix_web::web;
use crate::api::common::rest::crud_rest_service::CRUDRestService;
use crate::api::common::rest::rest_controller::RestController;
use crate::core::queuemanagement::rest::queue_management_service_impl::QueueManagementServiceImpl;

pub struct QueueRestController;

impl RestController for QueueRestController {
    fn scope(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/queue/search/")
                .route(web::post().to(QueueManagementServiceImpl::search))
        ).service(
            web::resource("/queue/")
                .route(web::post().to(QueueManagementServiceImpl::save))
        ).service(
            web::resource("/queue/{id}/")
                .route(web::get().to(QueueManagementServiceImpl::get))
                .route(web::delete().to(QueueManagementServiceImpl::delete))
        );
    }
}
