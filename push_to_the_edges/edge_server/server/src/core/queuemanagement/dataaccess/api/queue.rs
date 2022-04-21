use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::api::common::dataaccess::api::entity::Entity;
use crate::core::queuemanagement::logic::api::queue_eto::QueueEto;
use crate::core::general::database::schema::dailyqueue;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName, Insertable, AsChangeset)]
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
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime
}


impl Entity<i64, QueueEto> for Queue {
}

impl From<QueueEto> for Queue {
    fn from(queue_eto: QueueEto) -> Self {
        Queue {
            id: queue_eto.id.unwrap(),
            modification_counter: 1,
            name: queue_eto.name,
            logo: queue_eto.logo,
            current_number: queue_eto.current_number,
            attention_time: queue_eto.attention_time,
            min_attention_time: queue_eto.min_attention_time,
            active: queue_eto.active,
            created_at: Utc::now().naive_utc()
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

