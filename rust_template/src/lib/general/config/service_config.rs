use actix_web::Scope;
use crate::lib::usermanagement;

pub struct ServiceConfig {
    pub user_scope: Scope
}

impl ServiceConfig {
    pub fn new(
        base_rest_url: &String
    ) -> ServiceConfig {
        ServiceConfig {
            user_scope: usermanagement::rest::config::service_config::get_service_scope(base_rest_url)
        }
    }
}