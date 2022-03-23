use serde::{Serialize, Deserialize};
use crate::lib::general::search::pageable::Pageable;
use crate::lib::general::search::string_search_config::StringSearchConfig;

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
    pub username_option: Option<StringSearchConfig>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: Option<bool>,
    pub name_option: Option<StringSearchConfig>,
    pub phone_number_option: Option<StringSearchConfig>,
    pub pageable: Pageable,
}
