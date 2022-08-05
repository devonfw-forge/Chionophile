use actix_web::Scope;

use crate::api::controller::{
    accesscode_management_controller::AccessCodeRestController,
    queue_management_controller::QueueRestController, rest_controller::RestController,
    visitor_management_controller::VisitorRestController,
};

pub struct ServiceConfig {
    pub visitor_scope: Scope,
    pub queue_scope: Scope,
    pub accesscode_scope: Scope,
}

impl ServiceConfig {
    pub fn new(base_rest_url: &String) -> ServiceConfig {
        ServiceConfig {
            visitor_scope: AccessCodeRestController::get_scope(base_rest_url),
            queue_scope: QueueRestController::get_scope(base_rest_url),
            accesscode_scope: VisitorRestController::get_scope(base_rest_url),
        }
    }
}
