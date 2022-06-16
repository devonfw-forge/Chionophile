use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::common::logic::entity_eto::EntityETO;
use chrono::NaiveDateTime;

#[derive(Deserialize, Serialize, Validate, Clone, Debug)]
#[serde(rename_all="camelCase")]
pub struct AccessCodeEto {
    pub id: Option<i64>,
    pub modification_counter: Option<i32>,
    pub ticket_number: Option<String>,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub visitor_id: i64,
    pub queue_id: i64,
}

impl EntityETO for AccessCodeEto {}