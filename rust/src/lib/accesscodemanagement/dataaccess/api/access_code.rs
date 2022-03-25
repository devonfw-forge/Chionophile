use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::accesscode;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Associations, Identifiable, QueryableByName)]
#[belongs_to((Queue, foreign_key="queueId"), (Visitor, foreign_key="visitorId"))]
#[serde(rename_all = "camelCase")]
#[table_name="accesscode"]
pub struct AccessCode {
    pub id: i64,
    pub modification_counter: i32,
    pub ticket_number: Option<String>,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub visitor_id: i64,
    pub queue_id: i64,
}

