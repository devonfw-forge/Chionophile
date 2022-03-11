use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::schema::access_codes;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Associations, Identifiable, QueryableByName)]
#[belongs_to((Queue, foreign_key="queueId"), (Visitor, foreign_key="visitorId"))]
#[serde(rename_all = "camelCase")]
#[table_name="access_codes"]
pub struct AccessCode {
    pub id: String,
    pub ticket_number: Option<String>,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub queue_id: Option<String>,
    pub visitor_id: Option<String>,
}