use serde::{Serialize, Deserialize};
use crate::input::http_methods::HttpMethod;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Test {
    pub method: HttpMethod,
    pub path: String,
    pub path_args: Option<Vec<String>>,
    pub content_type: Option<String>,
    pub body: Option<String>,
    pub should_delete: Option<bool>,
    pub body_field_for_delete: Option<String>,
    pub delete_end_slash: Option<bool>
}