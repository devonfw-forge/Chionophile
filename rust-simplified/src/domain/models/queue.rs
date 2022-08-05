use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::domain::database::schema::dailyqueue;
use crate::domain::models::entity::Entity;
use crate::{api::model::queue_eto::QueueEto, domain::models::new_queue::NewQueue};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
#[serde(rename_all = "camelCase")]
#[table_name = "dailyqueue"]
pub struct Queue {
    pub id: i64,
    pub modification_counter: i32,
    pub name: Option<String>,
    pub logo: Option<String>,
    pub current_number: Option<String>,
    pub attention_time: Option<NaiveDateTime>,
    pub min_attention_time: NaiveDateTime,
    pub active: bool,
}

impl Entity<i64, NewQueue, QueueEto> for Queue {
    fn from_insert(id: i64, new_queue: NewQueue) -> Self {
        Queue {
            id,
            modification_counter: 1,
            name: new_queue.name,
            logo: new_queue.logo,
            current_number: new_queue.current_number,
            attention_time: new_queue.attention_time,
            min_attention_time: new_queue.min_attention_time,
            active: new_queue.active,
        }
    }
}

impl Into<QueueEto> for Queue {
    fn into(self) -> QueueEto {
        QueueEto {
            id: Option::from(self.id),
            modification_counter: Option::from(self.modification_counter),
            name: self.name,
            logo: self.logo,
            current_number: self.current_number,
            attention_time: self.attention_time,
            min_attention_time: self.min_attention_time,
            active: self.active,
        }
    }
}
