use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::lib::accesscodemanagement::dataaccess::api::access_code::AccessCode;

#[derive(Serialize, Deserialize)]
pub struct AccessCodeEto {
    pub id: Option<String>,
    pub ticket_number: Option<String>,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub queue_id: Option<String>,
    pub visitor_id: Option<String>,
}

impl AccessCodeEto {
    pub fn from(access_code: AccessCode) -> AccessCodeEto {
        AccessCodeEto {
            id: Some(access_code.id),
            ticket_number: access_code.ticket_number,
            creation_time: access_code.creation_time,
            start_time: access_code.start_time,
            end_time: access_code.end_time,
            queue_id: access_code.queue_id,
            visitor_id: access_code.visitor_id
        }
    }
}
