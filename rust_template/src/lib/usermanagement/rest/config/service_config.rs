use actix_web::{Scope, web};
use crate::lib::usermanagement::rest::user_management_controller::user_scope;

pub fn get_service_scope(base_rest_url: &String) -> Scope {
    let url = base_rest_url.to_owned() + "usermanagement/v1";
    web::scope(&url).configure(user_scope)
}