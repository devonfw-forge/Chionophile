use serde::{Deserialize, Serialize};
use crate::common::dataaccess::entity::Entity;
use crate::accesscode::logic::api::accesscode_eto::AccessCodeEto;
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodeEntity {
    pub id: i64,
    pub modification_counter: i32,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub visitor_id: i64,
    pub queue_id: i64,
}

impl Into<AccessCodeEto> for AccessCodeEntity {
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

impl Entity<i64, AccessCodeEto> for AccessCodeEntity {}

impl AccessCodeEntity{
    fn generate_ticket_number(&self) -> String {
        let mut ticket = self.id.to_string();
        while ticket.chars().count() < 3 {
            ticket.insert(0, '0');
        }
        ticket.insert(0, 'Q');
        ticket
    }
}