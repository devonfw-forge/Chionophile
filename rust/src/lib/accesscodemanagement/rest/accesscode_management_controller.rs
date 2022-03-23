use actix_web::web;
use crate::lib::accesscodemanagement::rest::accesscode_management_service::*;

pub fn access_code_scope(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/accesscode/")
            .route(web::post().to(save_accesscode))
    ).service(
        web::resource("/accesscode/{id}/")
            //.route(web::get().to(get_access_code))
            .route(web::delete().to(delete_accesscode))
    ).service(
        web::resource("/accesscode/search")
            .route(web::post().to(find_accesscode_etos))
    ) .service(
        web::resource("/accesscode/cto/search")
            .route(web::post().to(find_accesscode_ctos))
    ).service(
        web::resource("/accesscode/cto/{id}")
            .route(web::get().to(get_accesscode_cto))
    );
}
