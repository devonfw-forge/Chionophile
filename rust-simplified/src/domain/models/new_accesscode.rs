use chrono::NaiveDateTime;
use crate::{domain::{database::schema::accesscode, models::new_entity::NewEntity}, api::model::accesscode_eto::AccessCodeEto};

#[derive(Debug, Clone, Insertable)]
#[table_name="accesscode"]
pub struct NewAccessCode {
    pub modification_counter: i32,
    pub creation_time: Option<NaiveDateTime>,
    pub start_time: Option<NaiveDateTime>,
    pub end_time: Option<NaiveDateTime>,
    pub visitor_id: i64,
    pub queue_id: i64,
}

impl NewEntity<AccessCodeEto> for NewAccessCode {}

impl From<AccessCodeEto> for NewAccessCode {
    fn from(accesscode_eto: AccessCodeEto) -> Self {
        NewAccessCode {
            modification_counter: 1,
            creation_time: accesscode_eto.creation_time,
            start_time: accesscode_eto.start_time,
            end_time: accesscode_eto.end_time,
            queue_id: accesscode_eto.queue_id,
            visitor_id: accesscode_eto.visitor_id
        }
    }
}
