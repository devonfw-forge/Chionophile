use serde::{Serialize, Deserialize};
use crate::common::search::criteria::Criteria;
use crate::common::search::pageable::Pageable;
use chrono::NaiveDateTime;

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all="camelCase")]
pub struct AccessCodeSearchCriteria {
    pub id: Option<i64>,
    pub modification_counter: Option<i32>,
    pub ticket_number: Option<String>,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub id_visitor: Option<i64>,
    pub id_queue: Option<i64>,
    pub pageable: Pageable,
}

impl Criteria for AccessCodeSearchCriteria {}
