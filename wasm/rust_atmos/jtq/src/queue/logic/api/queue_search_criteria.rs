use serde::{Serialize, Deserialize};
use crate::common::search::criteria::Criteria;
use crate::common::search::pageable::Pageable;
use chrono::NaiveDateTime;

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all="camelCase")]
pub struct QueueSearchCriteria {
    pub modification_counter: Option<i32>,
    pub name: Option<String>,
    pub logo: Option<String>,
    pub current_number: Option<String>,
    pub attention_time: Option<NaiveDateTime>,
    pub min_attention_time: Option<NaiveDateTime>,
    pub active: Option<bool>,
    pub pageable: Pageable,
}

impl Criteria for QueueSearchCriteria {}
