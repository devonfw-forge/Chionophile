use actix_web::web;

use crate::api::controller::{
    crud_rest_service::CRUDRestService, rest_controller::RestController,
    visitor_management_service_impl::VisitorManagementServiceImpl,
};

pub struct VisitorRestController;

impl RestController for VisitorRestController {
    fn scope(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/visitor/search/")
                .route(web::post().to(VisitorManagementServiceImpl::search)),
        )
        .service(
            web::resource("/visitor/").route(web::post().to(VisitorManagementServiceImpl::save)),
        )
        .service(
            web::resource("/visitor/{id}/")
                .route(web::get().to(VisitorManagementServiceImpl::get))
                .route(web::delete().to(VisitorManagementServiceImpl::delete)),
        );
    }

    fn get_scope(base_url: &String) -> actix_web::Scope {
        let url = base_url.to_owned() + "visitormanagement/v1";
        web::scope(&url).configure(VisitorRestController::scope)
    }
}
