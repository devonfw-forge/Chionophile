use serde::{Serialize, Deserialize};
use crate::common::search::criteria::Criteria;
use crate::common::search::pageable::Pageable;

#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all="camelCase")]
pub struct AccessCodeSearchCriteria {
    pub username: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: Option<bool>,
    pub user_type: Option<bool>,
    pub pageable: Pageable,
}

impl Criteria for AccessCodeSearchCriteria {}
