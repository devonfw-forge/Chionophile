use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::api::common::logic::api::criteria::Criteria;
use crate::core::general::search::pageable::Pageable;

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueSearchCriteria {
    pub name : Option<String>,
    pub logo: Option<String>,
    pub current_number: Option<String>,
    pub attention_time: Option<NaiveDateTime>,
    pub min_attention_time: Option<NaiveDateTime>,
    pub active: Option<bool>,
    pub pageable: Pageable,
}


impl Criteria for QueueSearchCriteria {}
