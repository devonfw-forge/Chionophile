use serde::{Serialize, Deserialize};

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodePostData {
    pub visitor_id: String,
    pub queue_id: String
}
