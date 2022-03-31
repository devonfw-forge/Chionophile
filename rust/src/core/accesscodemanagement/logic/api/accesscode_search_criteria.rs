use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::api::common::logic::api::criteria::Criteria;
use crate::core::general::search::pageable::Pageable;

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodeSearchCriteria {
    pub ticket_number : Option<String>,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub visitor_id: Option<i64>,
    pub queue_id: Option<i64>,
    pub pageable: Pageable,
}

impl Criteria for AccessCodeSearchCriteria {}
