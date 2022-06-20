use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::common::dataaccess::entity::Entity;
use crate::queue::logic::api::queue_eto::QueueEto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueEntity {
    pub id: i64,
    #[serde(rename = "modificationcounter")]
    pub modification_counter: i32,
    pub name: Option<String>,
    pub logo: Option<String>,
    #[serde(rename = "currentnumber")]
    pub current_number: Option<String>,
    #[serde(rename = "attentiontime")]
    pub attention_time: Option<NaiveDateTime>,
    #[serde(rename = "minattentiontime")]
    pub min_attention_time: NaiveDateTime,
    pub active: bool,
}

impl Into<QueueEto> for QueueEntity {
    fn into(self) -> QueueEto {
        QueueEto {
            id: Some(self.id),
            modification_counter: Option::from(self.modification_counter),
            name: Option::from(self.name.unwrap()),
            logo: Option::from(self.logo.unwrap()),
            current_number: Option::from(self.current_number.unwrap()),
            attention_time: Option::from(self.attention_time.unwrap()),
            min_attention_time: self.min_attention_time,
            active: self.active
        }
    }
}

impl Entity<i64, QueueEto> for QueueEntity {}