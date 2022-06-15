use crate::common::logic::service::Service;
use crate::common::search::pageable::Pageable;
use crate::common::search::search_result::SearchResult;
use crate::visitor::logic::api::visitor_eto::VisitorEto;
use crate::visitor::logic::api::visitor_search_criteria::VisitorSearchCriteria;

pub struct VisitorService;

impl Service<VisitorEto, VisitorSearchCriteria, i64> for VisitorService {
    fn get_by_id(id: i64) -> VisitorEto {
        VisitorEto {
            id: None,
            modification_counter: None,
            username: None,
            name: None,
            phone_number: None,
            password: None,
            accepted_commercial: None,
            accepted_terms: false,
            user_type: None
        }
    }

    fn search(search_criteria: VisitorSearchCriteria) -> SearchResult<VisitorEto> {
        SearchResult {
            content: vec![],
            pageable: Pageable {
                page_size: 0,
                page_number: 0,
                sort: None
            },
            total_elements: 0
        }
    }

    fn delete(id: i64) -> i64 {
        id
    }

    fn create(eto: VisitorEto) -> VisitorEto {
        eto
    }
}