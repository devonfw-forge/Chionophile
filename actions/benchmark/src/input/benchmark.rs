use crate::input::request_group::RequestGroup;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Benchmark {
    pub name: String,
    pub request_groups: Vec<RequestGroup>
}