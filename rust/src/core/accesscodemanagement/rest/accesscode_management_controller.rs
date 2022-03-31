use actix_web::web;
use crate::api::accesscodemanagement::rest::api::accesscode_management_service::AccessCodeManagementService;
use crate::api::common::rest::crud_rest_service::CRUDRestService;
use crate::api::common::rest::rest_controller::RestController;
use crate::core::accesscodemanagement::rest::accesscode_management_service_impl::AccessCodeManagementServiceImpl;

pub struct AccessCodeRestController;

impl RestController for AccessCodeRestController {
    fn scope(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/accesscode/")
                .route(web::post().to(AccessCodeManagementServiceImpl::save))
        ).service(
            web::resource("/accesscode/{id}/")
                .route(web::delete().to(AccessCodeManagementServiceImpl::delete))
                .route(web::get().to(AccessCodeManagementServiceImpl::get))
        ).service(
            web::resource("/accesscode/search")
                .route(web::post().to(AccessCodeManagementServiceImpl::search))
        ).service(
            web::resource("/accesscode/cto/search")
                .route(web::post().to(AccessCodeManagementServiceImpl::find_accesscode_ctos))
        ).service(
            web::resource("/accesscode/cto/{id}")
                .route(web::get().to(AccessCodeManagementServiceImpl::find_accesscode_cto))
        );
    }
}
