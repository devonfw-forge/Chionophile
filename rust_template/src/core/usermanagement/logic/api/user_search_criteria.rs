use serde::{Serialize, Deserialize};
use crate::api::common::logic::api::criteria::Criteria;
use crate::core::general::search::pageable::Pageable;

/*
    Here we define all the possible filters when searching for a user.
    This will be sent by the FrontEnd, along with a pageable.
*/
#[derive(Deserialize, Clone, Serialize)]
#[serde(rename_all="camelCase")]
pub struct UserSearchCriteria {
    pub username: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: Option<bool>,
    pub pageable: Pageable,
}

impl Criteria for UserSearchCriteria {}
