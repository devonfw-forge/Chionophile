use actix_web::Scope;
use crate::core::accesscodemanagement;
use crate::core::visitormanagement;
use crate::core::queuemanagement;

pub struct ServiceConfig {
    pub visitor_scope: Scope,
    pub queue_scope: Scope,
    pub accesscode_scope: Scope
}

impl ServiceConfig {
    pub fn new(
        base_rest_url: &String
    ) -> ServiceConfig {
        ServiceConfig {
            visitor_scope: visitormanagement::rest::config::service_config::get_service_scope(base_rest_url),
            queue_scope: queuemanagement::rest::config::service_config::get_service_scope(base_rest_url),
            accesscode_scope: accesscodemanagement::rest::config::service_config::get_service_scope(base_rest_url)
        }
    }
}