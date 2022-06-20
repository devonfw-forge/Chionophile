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
    pub id_visitor: i64,
    pub id_queue: i64,
}

impl EntityETO for AccessCodeEto {}