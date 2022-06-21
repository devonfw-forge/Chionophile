use serde::{Serialize, Deserialize};
use crate::accesscode::logic::api::accesscode_eto::AccessCodeEto;
use crate::visitor::logic::api::visitor_eto::VisitorEto;
use crate::queue::logic::api::queue_eto::QueueEto;
// use crate::common::logic::entity_eto::EntityETO;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodeCto {
    pub access_code: AccessCodeEto,
    pub queue: QueueEto,
    pub visitor: VisitorEto
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessCodeQueryResult {
    //Accesscode
    #[serde(rename = "id")]
    pub access_code_id: i64,
    #[serde(rename = "modificationcounter")]
    pub modification_counter_access_code: i32,
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
    //Visitor
    #[serde(rename = "visitorid")]
    pub visitor_id: i64,
    #[serde(rename = "visitormodificationcounter")]
    pub modification_counter_visitor: Option<i32>,
    pub username: Option<String>,
    #[serde(rename = "visitorname")]
    pub visitor_name: Option<String>,
    #[serde(rename = "phonenumber")]
    pub phone_number: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "acceptedcommercial")]
    pub accepted_commercial: Option<bool>,
    #[serde(rename = "acceptedterms")]
    pub accepted_terms: bool,
    #[serde(rename = "usertype")]
    pub user_type: Option<bool>,
    //Queue
    #[serde(rename = "queueid")]
    pub queue_id: i64,
    #[serde(rename = "queuemodificationcounter")]
    pub modification_counter: Option<i32>,
    #[serde(rename = "name")]
    pub queue_name: Option<String>,
    pub logo: Option<String>,
    #[serde(rename = "currentnumber")]
    pub current_number: Option<String>,
    #[serde(rename = "attentiontime")]
    pub attention_time: Option<String>,
    #[serde(rename = "minattentiontime")]
    pub min_attention_time: Option<String>,
    pub active: bool,
}

impl Into<AccessCodeCto> for AccessCodeQueryResult {
    fn into(self) -> AccessCodeCto {
        AccessCodeCto {
            access_code: AccessCodeEto{
                id: Some(self.access_code_id),
                modification_counter: Option::from(self.modification_counter_access_code),
                ticket_number: Option::from(self.generate_ticket_number()),
                creation_time: Option::from(self.creation_time),
                start_time: Option::from(self.start_time),
                end_time: Option::from(self.end_time),
                visitor_id: self.id_visitor,
                queue_id: self.id_queue
            },
            visitor: VisitorEto{
                id: Some(self.visitor_id),
                modification_counter: Option::from(self.modification_counter_visitor),
                username: Option::from(self.username),
                name: Option::from(self.visitor_name),
                phone_number: Option::from(self.phone_number),
                password: Option::from(self.password),
                accepted_commercial: Option::from(self.accepted_commercial),
                accepted_terms: self.accepted_terms,
                user_type: Option::from(self.user_type)
            },
            queue: QueueEto{
                id: Some(self.queue_id),
                modification_counter: Option::from(self.modification_counter),
                name: Option::from(self.queue_name),
                logo: Option::from(self.logo),
                current_number: Option::from(self.current_number),
                attention_time: Option::from(self.attention_time),
                min_attention_time: self.min_attention_time.unwrap(),
                active: self.active
            }
        }
    }
}

impl AccessCodeQueryResult{
    fn generate_ticket_number(&self) -> String {
        let mut ticket = self.access_code_id.to_string();
        while ticket.chars().count() < 3 {
            ticket.insert(0, '0');
        }
        ticket.insert(0, 'Q');
        ticket
    }
}