use actix_web::web;

use crate::api::controller::crud_rest_service::CRUDRestService;

use super::{
    accesscode_management_service_impl::AccessCodeManagementServiceImpl,
    rest_controller::RestController,
};

pub struct AccessCodeRestController;

impl RestController for AccessCodeRestController {
    fn scope(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/accesscode/")
                .route(web::post().to(AccessCodeManagementServiceImpl::save)),
        )
        .service(
            web::resource("/accesscode/search/")
                .route(web::post().to(AccessCodeManagementServiceImpl::search)),
        )
        .service(
            web::resource("/accesscode/{id}/")
                .route(web::delete().to(AccessCodeManagementServiceImpl::delete))
                .route(web::get().to(AccessCodeManagementServiceImpl::get)),
        )
        .service(
            web::resource("/accesscode/cto/search/")
                .route(web::post().to(AccessCodeManagementServiceImpl::find_accesscode_ctos)),
        )
        .service(
            web::resource("/accesscode/cto/{id}/")
                .route(web::get().to(AccessCodeManagementServiceImpl::find_accesscode_cto)),
        );
    }

    fn get_scope(base_url: &String) -> actix_web::Scope {
        let url = base_url.to_owned() + "accesscodemanagement/v1";
        web::scope(&url).configure(AccessCodeRestController::scope)
    }
}
