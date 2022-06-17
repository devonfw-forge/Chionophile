use serde::{Deserialize, Serialize};
use crate::common::dataaccess::entity::Entity;
use crate::visitor::logic::api::visitor_eto::VisitorEto;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisitorEntity {
    pub id: i64,
    #[serde(rename = "modificationcounter")]
    pub modification_counter: i32,
    pub username: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    #[serde(rename = "phonenumber")]
    pub phone_number: Option<String>,
    #[serde(rename = "acceptedcommercial")]
    pub accepted_commercial: Option<bool>,
    #[serde(rename = "acceptedterms")]
    pub accepted_terms: bool,
    #[serde(rename = "usertype")]
    pub user_type: Option<bool>,
}

impl Into<VisitorEto> for VisitorEntity {
    fn into(self) -> VisitorEto {
        VisitorEto {
            id: Some(self.id),
            modification_counter: Option::from(self.modification_counter),
            username: Option::from(self.username.unwrap()),
            name: Option::from(self.name.unwrap()),
            phone_number: Option::from(self.phone_number.unwrap()),
            password: Option::from(self.password.unwrap()),
            accepted_commercial: self.accepted_commercial,
            accepted_terms: self.accepted_terms,
            user_type: self.user_type
        }
    }
}

impl Entity<i64, VisitorEto> for VisitorEntity {}