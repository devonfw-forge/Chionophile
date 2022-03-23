use chrono::NaiveDateTime;
use crate::lib::queuemanagement::logic::api::queue_eto::QueueEto;
use crate::schema::dailyqueue;

#[derive(Debug, Clone, Insertable)]
#[table_name="dailyqueue"]
pub struct NewQueue {
    pub modification_counter: i32,
    pub name: Option<String>,
    pub logo: Option<String>,
    pub current_number: Option<String>,
    pub attention_time: Option<NaiveDateTime>,
    pub min_attention_time: NaiveDateTime,
    pub active: bool,
    pub customers: i32,
}

impl NewQueue {
    pub fn from(
        queue_eto: QueueEto
    ) -> NewQueue {
        NewQueue {
            modification_counter: 1,
            name: queue_eto.name,
            logo: queue_eto.logo,
            current_number: queue_eto.current_number,
            attention_time: queue_eto.attention_time,
            min_attention_time: queue_eto.min_attention_time,
            active: queue_eto.active,
            customers: queue_eto.customers
        }
    }
}
