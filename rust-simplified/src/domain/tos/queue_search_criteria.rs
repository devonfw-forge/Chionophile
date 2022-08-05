use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

use crate::domain::search::pageable::Pageable;
use crate::domain::tos::criteria::Criteria;

#[derive(Deserialize, Clone, Serialize, Criteria)]
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

