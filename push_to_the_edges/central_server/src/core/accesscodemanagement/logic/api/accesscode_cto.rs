use serde::Serialize;
use crate::core::accesscodemanagement::dataaccess::api::accesscode::AccessCode;
use crate::core::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::core::queuemanagement::dataaccess::api::queue::Queue;
use crate::core::queuemanagement::logic::api::queue_eto::QueueEto;
use crate::core::visitormanagement::dataaccess::api::visitor::Visitor;
use crate::core::visitormanagement::logic::api::visitor_eto::VisitorEto;

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodeCto {
    pub access_code: AccessCodeEto,
    pub queue: QueueEto,
    pub visitor: VisitorEto
}

impl AccessCodeCto {
    pub fn from_query_result(
        mut query_result: (AccessCode, Visitor, Queue),
    ) -> AccessCodeCto {
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
