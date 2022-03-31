use crate::api::common::rest::crud_rest_service::CRUDRestService;
use crate::core::visitormanagement::logic::api::visitor_eto::VisitorEto;
use crate::core::visitormanagement::logic::api::visitor_search_criteria::VisitorSearchCriteria;

/*
    This is the trait that defines the methods of the Visitor Endpoint. In this case,
    it's enough with the methods provided by the CRUDRestService trait, but all other methods
    not included there, can be defined in this file.
*/
pub trait VisitorManagementService:  CRUDRestService<i64, VisitorEto, VisitorSearchCriteria, VisitorEto> {}
