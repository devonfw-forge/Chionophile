use chrono::NaiveDateTime;
use crate::schema::dailyqueue;
use serde::{Deserialize, Serialize};
use crate::lib::queuemanagement::dataacess::api::new_queue::NewQueue;
use crate::lib::queuemanagement::logic::api::queue_eto::QueueEto;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable, QueryableByName)]
#[serde(rename_all = "camelCase")]
#[table_name="dailyqueue"]
pub struct Queue {
    pub id: i64,
    pub modification_counter: i32,
    pub name: Option<String>,
    pub logo: Option<String>,
    pub current_number: Option<String>,
    pub attention_time: Option<NaiveDateTime>,
    pub min_attention_time: NaiveDateTime,
    pub active: bool,
    pub customers: i32,
}

impl Queue {
    pub fn from(
        queue_eto: QueueEto
    ) -> Queue {
        let mut queue = Queue {
            id: -1,
            modification_counter: 1,
            name: queue_eto.name,
            logo: queue_eto.logo,
            current_number: queue_eto.current_number,
            attention_time: queue_eto.attention_time,
            min_attention_time: queue_eto.min_attention_time,
            active: queue_eto.active,
            customers: queue_eto.customers
        };

        if let Some(id) = queue_eto.id {
            queue.id = id;
        }
        queue
    }
    pub fn from_insert(
        id: i64,
        new_queue: NewQueue
    ) -> Queue {
        Queue {
            id,
            modification_counter: 1,
            name: new_queue.name,
            logo: new_queue.logo,
            current_number: new_queue.current_number,
            attention_time: new_queue.attention_time,
            min_attention_time: new_queue.min_attention_time,
            active: new_queue.active,
            customers: new_queue.customers
        }
    }
}
