use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::lib::general::pageable::Pageable;

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueSearchCriteria {
    pub name : Option<String>,
    pub logo: Option<String>,
    pub current_number: Option<String>,
    pub attention_time: Option<NaiveDateTime>,
    pub min_attention_time: Option<NaiveDateTime>,
    pub active: Option<bool>,
    pub customers: Option<i32>,
    pub pageable: Pageable,
}
