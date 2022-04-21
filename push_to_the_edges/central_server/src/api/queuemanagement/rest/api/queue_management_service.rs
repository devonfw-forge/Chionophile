use crate::api::common::rest::crud_rest_service::CRUDRestService;
use crate::core::queuemanagement::logic::api::queue_eto::QueueEto;
use crate::core::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;

/*
    This is the trait that defines the methods of the Queue Endpoint. In this case,
    it's enough with the methods provided by the CRUDRestService trait, but all other methods
    not included there, can be defined in this file.
*/
pub trait QueueManagementService:  CRUDRestService<i64, QueueEto, QueueSearchCriteria, QueueEto> {}
