use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueueEto {
    pub id: Option<i64>,
    pub name: Option<String>,
    pub logo: Option<String>,
    pub current_number: Option<String>,
    pub attention_time: Option<DateTime<Utc>>,
    pub min_attention_time: DateTime<Utc>,
    pub active: bool,
}

impl QueueEto {
    pub fn generate_test_queue() -> QueueEto {
        QueueEto {
            id: None,
            name: Some("TestQueue".to_string()),
            logo: Some("TestLogo".to_string()),
            current_number: Some("Q000".to_string()),
            attention_time: None,
            min_attention_time: Utc::now(),
            active: true,
        }
    }
}
