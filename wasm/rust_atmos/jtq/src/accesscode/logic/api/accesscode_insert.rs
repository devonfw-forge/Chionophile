use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::common::logic::entity_eto::EntityETO;

#[derive(Deserialize, Serialize, Validate, Clone, Debug)]
#[serde(rename_all="camelCase")]
pub struct AccessCodeInsert {
    pub visitor_id: i64,
    pub queue_id: i64,
}

impl EntityETO for AccessCodeInsert {}