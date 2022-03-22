use crate::lib::accesscodemanagement::dataaccess::api::access_code::AccessCode;
use crate::lib::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::lib::queuemanagement::dataacess::api::queue::Queue;
use crate::lib::queuemanagement::logic::api::queue_eto::QueueEto;
use crate::lib::visitormanagement::dataacess::api::visitor::Visitor;
use crate::lib::visitormanagement::logic::api::visitor_eto::VisitorEto;
use serde::Serialize;

#[derive(Serialize)]
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
            access_code: AccessCodeEto::from(query_result.0),
            visitor: VisitorEto::from(query_result.1),
            queue: QueueEto::from(query_result.2),
        }
    }
}
