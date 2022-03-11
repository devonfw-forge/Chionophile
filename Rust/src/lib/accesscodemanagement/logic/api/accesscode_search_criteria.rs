use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::lib::general::pageable::Pageable;

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodeSearchCriteria {
    pub ticket_number : Option<String>,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub visitor_id: Option<String>,
    pub queue_id: Option<String>,
    pub pageable: Pageable,
}
