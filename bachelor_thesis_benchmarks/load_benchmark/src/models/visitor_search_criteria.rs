use serde::{Serialize, Deserialize};
use crate::models::pageable::Pageable;

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all="camelCase")]
pub struct VisitorSearchCriteria {
    pub username: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: Option<bool>,
    pub user_type: Option<bool>,
    pub pageable: Pageable,
}


impl VisitorSearchCriteria {
    pub fn generate_test_search_criteria(
        page_size: i32,
        username: Option<String>,
        password: Option<String>
    ) -> VisitorSearchCriteria {
        VisitorSearchCriteria {
            pageable: Pageable {
                page_size,
                page_number: 0,
                sort: Some(vec![])
            },
            username,
            name: None,
            phone_number: None,
            password,
            accepted_commercial: None,
            accepted_terms: None,
            user_type: None
        }
    }
}
