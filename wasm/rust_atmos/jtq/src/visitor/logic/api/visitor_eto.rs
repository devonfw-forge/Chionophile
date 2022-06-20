use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::common::logic::entity_eto::EntityETO;

#[derive(Deserialize, Serialize, Validate, Clone, Debug)]
#[serde(rename_all="camelCase")]
pub struct VisitorEto {
    pub id: Option<i64>,
    #[serde(default = "default_modification_counter")]
    pub modification_counter: Option<i32>,
    #[validate(email(code="mail", message="Email format not valid"))]
    pub username: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: bool,
    pub user_type: Option<bool>,
}

impl EntityETO for VisitorEto {}

fn default_modification_counter() -> Option<i32> {
    Option::from(1)
}