use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::common::logic::entity_eto::EntityETO;

#[derive(Deserialize, Serialize, Validate, Clone, Debug)]
#[serde(rename_all="camelCase")]
pub struct AccessCodeEto {
    pub id: Option<i64>,
    pub modification_counter: Option<i32>,
    pub ticket_number: Option<String>,
    pub creation_time: Option<String>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
    pub visitor_id: i64,
    pub queue_id: i64,
}

impl EntityETO for AccessCodeEto {}