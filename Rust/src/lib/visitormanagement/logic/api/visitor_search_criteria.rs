use serde::{Serialize, Deserialize};
use crate::lib::general::pageable::Pageable;

#[derive(Deserialize, Clone, Serialize)]
pub struct VisitorSearchCriteria {
    pub pageable: Pageable,
    pub username: Option<String>,
    pub password: Option<String>
}
