use chrono::{NaiveDateTime};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct AccessCodeEto {
    pub id: Option<i64>,
    pub ticket_number: Option<String>,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub queue_id: i64,
    pub visitor_id: i64,
}
