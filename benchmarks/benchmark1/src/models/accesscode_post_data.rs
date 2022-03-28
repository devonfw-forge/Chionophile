use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodePostData {
    pub visitor_id: i64,
    pub queue_id: i64
}

impl AccessCodePostData {
    pub fn new(
        visitor_id: i64,
        queue_id: i64
    ) -> AccessCodePostData {
        AccessCodePostData {
            visitor_id,
            queue_id
        }
    }
}