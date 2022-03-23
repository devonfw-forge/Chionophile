use serde::{Serialize, Deserialize};

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodePostData {
    pub visitor_id: i64,
    pub queue_id: i64
}
