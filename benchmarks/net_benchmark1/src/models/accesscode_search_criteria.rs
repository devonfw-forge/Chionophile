use chrono::{DateTime,Utc};
use serde::{Serialize, Deserialize};
use crate::models::pageable::Pageable;

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodeSearchCriteria {
    pub ticket_number : Option<String>,
    pub creation_time: Option<DateTime<Utc>>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub visitor_id: Option<i64>,
    pub queue_id: Option<i64>,
    pub pageable: Pageable,
}

impl AccessCodeSearchCriteria {
    pub fn generate_test_struct(page_size: i32) -> Self {
        AccessCodeSearchCriteria {
            ticket_number: None,
            creation_time: None,
            start_time: None,
            end_time: None,
            visitor_id: None,
            queue_id: None,
            pageable: Pageable {
                page_size,
                page_number: 0,
                sort: None
            }
        }
    }
}

