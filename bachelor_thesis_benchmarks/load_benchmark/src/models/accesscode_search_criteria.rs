use crate::models::pageable::Pageable;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodeSearchCriteria {
    pub ticket_number: Option<String>,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub visitor_id: Option<i64>,
    pub queue_id: Option<i64>,
    pub pageable: Pageable,
}

impl AccessCodeSearchCriteria {
    pub fn generate_test_struct(page_size: i32, creation_time: NaiveDateTime, visitor_id: i64) -> Self {
        AccessCodeSearchCriteria {
            ticket_number: None,
            creation_time: Some(creation_time),
            start_time: None,
            end_time: None,
            visitor_id: Some(visitor_id),
            queue_id: None,
            pageable: Pageable {
                page_size,
                page_number: 0,
                sort: None,
            },
        }
    }
}
