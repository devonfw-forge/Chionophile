use actix_web::Scope;
use crate::core::usermanagement;

/*
    Actix requires that each scope is registered individually. This struct is made to store
    all of your application scopes and just import and use this in the main function.
*/
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