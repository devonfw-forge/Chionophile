use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    DELETE
}