use actix_web::web;
use crate::api::common::rest::crud_rest_service::CRUDRestService;
use crate::api::common::rest::rest_controller::RestController;
use crate::core::usermanagement::rest::user_management_service_impl::UserManagementServiceImpl;
/*
    The scope it's the endpoint's controller. It defines the URLs and redirects 
    the HTTP request to the correct method in the endpoint's service.
*/
pub struct UserRestController;

impl RestController for UserRestController {
    fn scope(cfg: &mut web::ServiceConfig) {
        cfg.service(
            web::resource("/user/search")
                .route(web::post().to(UserManagementServiceImpl::search))
        ).service(
            web::resource("/user")
                .route(web::post().to(UserManagementServiceImpl::save))
        ).service(
            web::resource("/user/{id}")
                .route(web::get().to(UserManagementServiceImpl::get))
                .route(web::delete().to(UserManagementServiceImpl::delete))
        );
    }
}