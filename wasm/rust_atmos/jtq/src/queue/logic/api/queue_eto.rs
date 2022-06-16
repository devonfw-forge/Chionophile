use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::common::logic::entity_eto::EntityETO;

#[derive(Deserialize, Serialize, Validate, Clone)]
#[serde(rename_all="camelCase")]
pub struct QueueEto {
    pub id: Option<i64>,
    pub modification_counter: Option<i32>,
    pub name: Option<String>,
    pub logo: Option<String>,
    pub current_number: Option<String>,
    pub attention_time: Option<NaiveDateTime>,
    pub min_attention_time: NaiveDateTime,
    pub active: bool,
}

impl EntityETO for QueueEto {}