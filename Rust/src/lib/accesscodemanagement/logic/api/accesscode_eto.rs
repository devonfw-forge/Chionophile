use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::lib::accesscodemanagement::dataaccess::api::access_code::AccessCode;
use crate::lib::accesscodemanagement::dataaccess::api::new_access_code::NewAccessCode;

#[derive(Serialize, Deserialize)]
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
    pub fn from_insert(id: i64, access_code: NewAccessCode) -> AccessCodeEto {
        AccessCodeEto {
            id: Some(id),
            ticket_number: access_code.ticket_number,
            creation_time: access_code.creation_time,
            start_time: access_code.start_time,
            end_time: access_code.end_time,
            queue_id: access_code.queue_id,
            visitor_id: access_code.visitor_id
        }
    }
}
