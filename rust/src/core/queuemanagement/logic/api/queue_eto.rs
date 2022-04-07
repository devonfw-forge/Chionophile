use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::api::common::logic::api::eto::EntityETO;
use crate::api::common::rest::api::saveable::Saveable;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QueueEto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub logo: Option<String>,
    pub current_number: Option<String>,
    pub attention_time: Option<NaiveDateTime>,
    pub min_attention_time: NaiveDateTime,
    pub active: bool,
}

impl EntityETO for QueueEto {}
impl Saveable for QueueEto {}
