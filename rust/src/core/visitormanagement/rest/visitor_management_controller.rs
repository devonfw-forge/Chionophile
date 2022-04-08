use actix_web::web;
use crate::api::common::rest::crud_rest_service::CRUDRestService;
use crate::api::common::rest::rest_controller::RestController;
use crate::core::visitormanagement::rest::visitor_management_service_impl::VisitorManagementServiceImpl;

pub struct VisitorRestController;

impl RestController for VisitorRestController {
    fn scope(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/visitor/search/")
                .route(web::post().to(VisitorManagementServiceImpl::search))
        ).service(
            web::resource("/visitor/")
                .route(web::post().to(VisitorManagementServiceImpl::save))
        ).service(
            web::resource("/visitor/{id}/")
                .route(web::get().to(VisitorManagementServiceImpl::get))
                .route(web::delete().to(VisitorManagementServiceImpl::delete))
        );
    }
}
