use chrono::NaiveDateTime;
use crate::lib::queuemanagement::dataacess::api::queue::{Queue};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueEto {
    pub id: Option<String>,
    pub name: Option<String>,
    pub logo: Option<String>,
    pub current_number: Option<String>,
    pub attention_time: Option<NaiveDateTime>,
    pub min_attention_time: NaiveDateTime,
    pub active: bool,
    pub customers: i32,
}

impl QueueEto {
    pub fn from(queue: Queue) -> QueueEto {
        QueueEto {
            id: Some(queue.id),
            name: queue.name,
            logo: queue.logo,
            current_number: queue.current_number,
            attention_time: queue.attention_time,
            min_attention_time: queue.min_attention_time,
            active: queue.active,
            customers: queue.customers
        }
    }
}
