use crate::input::request::Request;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestGroup {
    pub requests: Vec<Request>,
    #[serde(skip_serializing)]
    pub base_path: Option<String>
}

impl RequestGroup {
    pub fn new(
        requests: Vec<Request>,
        base_path: String
    ) -> Self {
        RequestGroup {
            requests,
            base_path: Option::from(base_path)
        }
    }
}