use serde::Serialize;

use crate::{
    api::model::{accesscode_eto::AccessCodeEto, queue_eto::QueueEto, visitor_eto::VisitorEto},
    domain::models::{accesscode::AccessCode, queue::Queue, visitor::Visitor},
};

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodeCto {
    pub access_code: AccessCodeEto,
    pub queue: QueueEto,
    pub visitor: VisitorEto,
}

impl AccessCodeCto {
    pub fn from_query_result(mut query_result: (AccessCode, Visitor, Queue)) -> AccessCodeCto {
        let access_code = query_result.0.clone();
        query_result.1.id = access_code.visitor_id;
        query_result.2.id = access_code.queue_id;
        AccessCodeCto {
            access_code: query_result.0.into(),
            visitor: query_result.1.into(),
            queue: query_result.2.into(),
        }
    }
}
