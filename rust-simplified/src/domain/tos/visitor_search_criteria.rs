use serde::{Serialize, Deserialize};

use crate::domain::search::pageable::Pageable;
use crate::domain::tos::criteria::Criteria;

#[derive(Deserialize, Clone, Serialize, Criteria)]
#[serde(rename_all="camelCase")]
pub struct VisitorSearchCriteria {
    pub username: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: Option<bool>,
    pub user_type: Option<bool>,
    pub pageable: Pageable,
}
