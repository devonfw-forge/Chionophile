use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::api::common::dataaccess::api::entity::Entity;
use crate::core::accesscodemanagement::dataaccess::api::new_accesscode::NewAccessCode;
use crate::core::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::core::general::database::schema::accesscode;

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


impl Entity<i64, NewAccessCode, AccessCodeEto> for AccessCode {
    fn from_insert(
        id: i64,
        new_accesscode: NewAccessCode
    ) -> Self {
        AccessCode {
            id,
            modification_counter: 1,
            ticket_number: new_accesscode.ticket_number,
            creation_time: new_accesscode.creation_time,
            start_time: new_accesscode.start_time,
            end_time: new_accesscode.end_time,
            queue_id: new_accesscode.queue_id,
            visitor_id: new_accesscode.visitor_id
        }
    }
}

impl Into<AccessCodeEto> for AccessCode {
    fn into(self) -> AccessCodeEto {
        AccessCodeEto {
            id: Some(self.id),
            ticket_number: self.ticket_number,
            creation_time: self.creation_time,
            start_time: self.start_time,
            end_time: self.end_time,
            queue_id: self.queue_id,
            visitor_id: self.visitor_id
        }
    }
}
