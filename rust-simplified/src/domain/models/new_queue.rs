use chrono::NaiveDateTime;

use crate::domain::database::schema::dailyqueue;
use crate::{api::model::queue_eto::QueueEto, domain::models::new_entity::NewEntity};

#[derive(Debug, Clone, Insertable)]
#[table_name = "dailyqueue"]
pub struct NewQueue {
    pub modification_counter: i32,
    pub name: Option<String>,
    pub logo: Option<String>,
    pub current_number: Option<String>,
    pub attention_time: Option<NaiveDateTime>,
    pub min_attention_time: NaiveDateTime,
    pub active: bool,
}

impl NewEntity<QueueEto> for NewQueue {}

impl From<QueueEto> for NewQueue {
    fn from(queue_eto: QueueEto) -> Self {
        NewQueue {
            modification_counter: 1,
            name: queue_eto.name,
            logo: queue_eto.logo,
            current_number: queue_eto.current_number,
            attention_time: queue_eto.attention_time,
            min_attention_time: queue_eto.min_attention_time,
            active: queue_eto.active,
        }
    }
}
