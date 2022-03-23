use serde::{Serialize, Deserialize};
use crate::lib::general::search::string_search_operator::StringSearchOperator;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all="camelCase")]
pub struct StringSearchConfig {
    pub ignore_case: Option<bool>,
    pub match_substring: Option<bool>,
    pub string_search_operator: Option<StringSearchOperator>
}