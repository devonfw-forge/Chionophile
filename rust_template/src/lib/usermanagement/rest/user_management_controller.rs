use actix_web::web;
use crate::lib::usermanagement::rest::user_management_service::{delete_user, get_user, save_user, search_user};

/*
    The scope it's the endpoint's controller. It defines the URLs and redirects 
    the HTTP request to the correct method in the endpoint's service.
*/
pub fn user_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user/search")
            .route(web::post().to(search_user))
    ).service(
        web::resource("/user")
            .route(web::post().to(save_user))
    ).service(
        web::resource("/user/{id}")
            .route(web::get().to(get_user))
            .route(web::delete().to(delete_user))
    );
}