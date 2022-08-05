use crate::api::model::eto::EntityETO;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, EntityETO)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodeEto {
    pub id: Option<i64>,
    pub modification_counter: Option<i32>,
    pub ticket_number: Option<String>,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub queue_id: i64,
    pub visitor_id: i64,
}
