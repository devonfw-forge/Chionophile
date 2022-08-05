use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

use crate::{api::model::accesscode_eto::AccessCodeEto, domain::models::{new_accesscode::NewAccessCode, entity::Entity}};
use crate::domain::database::schema::accesscode;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Associations, Identifiable, QueryableByName)]
#[belongs_to((Queue, foreign_key="queueId"), (Visitor, foreign_key="visitorId"))]
#[serde(rename_all = "camelCase")]
#[table_name="accesscode"]
pub struct AccessCode {
    pub id: i64,
    pub modification_counter: i32,
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
            modification_counter: Option::from(self.modification_counter),
            ticket_number: Option::from(self.generate_ticket_number()),
            creation_time: self.creation_time,
            start_time: self.start_time,
            end_time: self.end_time,
            queue_id: self.queue_id,
            visitor_id: self.visitor_id
        }
    }
}

impl AccessCode {
    fn generate_ticket_number(&self) -> String {
        let mut ticket = self.id.to_string();
        while ticket.chars().count() < 3 {
            ticket.insert(0, '0');
        }
        ticket.insert(0, 'Q');
        ticket
    }
}
