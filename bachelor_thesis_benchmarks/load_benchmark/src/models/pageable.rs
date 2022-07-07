use serde::{Serialize, Deserialize};
use crate::models::sort::Sort;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct Pageable {
    pub page_size: i32,
    pub page_number: i32,
    pub sort: Option<Vec<Sort>>
}
