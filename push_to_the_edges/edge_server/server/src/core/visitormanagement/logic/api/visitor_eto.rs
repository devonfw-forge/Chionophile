use serde::{Serialize, Deserialize};
use validator::Validate;
use crate::api::common::logic::api::eto::EntityETO;
use crate::api::common::rest::api::saveable::Saveable;

#[derive(Deserialize, Serialize, Validate, Clone)]
#[serde(rename_all="camelCase")]
pub struct VisitorEto {
    pub id: Option<i64>,
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
impl Saveable for VisitorEto {}