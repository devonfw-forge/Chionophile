use serde::{Serialize, Deserialize};
use crate::models::sort::Sort;

#[derive(Deserialize, Serialize)]
pub struct Pageable {
    pub pageSize: i32,
    pub pageNumber: i32,
    pub sort: Option<Vec<Sort>>
}
