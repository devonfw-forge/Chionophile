use serde::{Deserialize, Serialize};
use crate::common::dataaccess::entity::Entity;
use crate::accesscode::logic::api::accesscode_eto::AccessCodeEto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessCodeEntity {
    pub id: i64,
    #[serde(rename = "modificationcounter")]
    pub modification_counter: i32,
    #[serde(rename = "creationtime")]
    pub creation_time: Option<String>,
    #[serde(rename = "starttime")]
    pub start_time: Option<String>,
    #[serde(rename = "endtime")]
    pub end_time: Option<String>,
    #[serde(rename = "idvisitor")]
    pub id_visitor: i64,
    #[serde(rename = "idqueue")]
    pub id_queue: i64,
}

impl Into<AccessCodeEto> for AccessCodeEntity {
    fn into(self) -> AccessCodeEto {
        AccessCodeEto {
            id: Some(self.id),
            modification_counter: Option::from(self.modification_counter),
            ticket_number: Option::from(self.generate_ticket_number()),
            creation_time: Option::from(self.creation_time),
            start_time: Option::from(self.start_time),
            end_time: Option::from(self.end_time),
            id_queue: self.id_queue,
            id_visitor: self.id_visitor
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