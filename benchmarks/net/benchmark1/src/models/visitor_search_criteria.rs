use serde::{Serialize, Deserialize};
use crate::models::pageable::Pageable;

#[derive(Deserialize, Serialize)]
pub struct VisitorSearchCriteria {
    pub pageable: Pageable,
    pub username: Option<String>,
    pub password: Option<String>
}

impl VisitorSearchCriteria {
    pub fn generate_test_search_criteria(
        page_size: i32,
        username: Option<String>,
        password: Option<String>
    ) -> VisitorSearchCriteria {
        VisitorSearchCriteria {
            pageable: Pageable {
                pageSize: page_size,
                pageNumber: 0,
                sort: Some(vec![])
            },
            username,
            password
        }
    }
}
