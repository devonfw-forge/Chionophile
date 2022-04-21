use actix_web::{Scope, web};
use crate::api::common::rest::rest_controller::RestController;
use crate::core::accesscodemanagement::rest::accesscode_management_controller::AccessCodeRestController;

/*
    This module is meant to return the complete url to the endpoint, and will be used in the
    ServiceConfig struct in core/api/service_config
*/
pub fn get_service_scope(base_rest_url: &String) -> Scope {
    let url = base_rest_url.to_owned() + "accesscodemanagement/v1";
    web::scope(&url).configure(AccessCodeRestController::scope)
}