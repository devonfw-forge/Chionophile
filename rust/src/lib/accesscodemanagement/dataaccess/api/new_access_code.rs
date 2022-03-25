use chrono::NaiveDateTime;
use crate::schema::accesscode;

#[derive(Debug, Clone, Insertable)]
#[table_name="accesscode"]
pub struct NewAccessCode {
    pub modification_counter: i32,
    pub ticket_number: Option<String>,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub visitor_id: i64,
    pub queue_id: i64,
}
