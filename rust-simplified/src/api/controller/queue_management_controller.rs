use actix_web::web;

use crate::api::controller::{
    crud_rest_service::CRUDRestService, queue_management_service_impl::QueueManagementServiceImpl,
    rest_controller::RestController,
};

pub struct QueueRestController;

impl RestController for QueueRestController {
    fn scope(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/queue/search/")
                .route(web::post().to(QueueManagementServiceImpl::search)),
        )
        .service(web::resource("/queue/").route(web::post().to(QueueManagementServiceImpl::save)))
        .service(
            web::resource("/queue/{id}/")
                .route(web::get().to(QueueManagementServiceImpl::get))
                .route(web::delete().to(QueueManagementServiceImpl::delete)),
        );
    }

    fn get_scope(base_url: &String) -> actix_web::Scope {
        let url = base_url.to_owned() + "queuemanagement/v1";
        web::scope(&url).configure(QueueRestController::scope)
    }
}
