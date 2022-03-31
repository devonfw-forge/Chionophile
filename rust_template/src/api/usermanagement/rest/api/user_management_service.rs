use crate::api::common::rest::crud_rest_service::CRUDRestService;
use crate::core::usermanagement::logic::api::user_eto::UserEto;
use crate::core::usermanagement::logic::api::user_search_criteria::UserSearchCriteria;

/*
    This is the trait that defines the methods of the User Endpoint. In this case,
    it's enough with the methods provided by the CRUDRestService trait, but all other methods
    not included there, can be defined in this file.
*/
pub trait UserManagementService:  CRUDRestService<i64, UserEto, UserSearchCriteria, UserEto> {}