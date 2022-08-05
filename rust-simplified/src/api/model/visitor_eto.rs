use crate::api::model::eto::EntityETO;
use crate::api::model::saveable::Saveable;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Validate, Clone, EntityETO, Saveable)]
#[serde(rename_all = "camelCase")]
pub struct VisitorEto {
    pub id: Option<i64>,
    pub modification_counter: Option<i32>,
    #[validate(email(code = "mail", message = "Email format not valid"))]
    pub username: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: bool,
    pub user_type: Option<bool>,
}
