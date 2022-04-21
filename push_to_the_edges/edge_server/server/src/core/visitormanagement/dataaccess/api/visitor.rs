use chrono::{NaiveDateTime, Utc};
use serde::{Serialize, Deserialize};
use crate::api::common::dataaccess::api::entity::Entity;
use crate::core::visitormanagement::logic::api::visitor_eto::VisitorEto;
use crate::core::general::database::schema::visitor;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName, Insertable, AsChangeset)]
#[serde(rename_all = "camelCase")]
#[table_name="visitor"]
pub struct Visitor {
    pub id: i64,
    pub modification_counter: i32,
    pub username: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: bool,
    pub user_type: Option<bool>,
    #[serde(skip_serializing)]
    pub created_at: NaiveDateTime
}

impl Entity<i64, VisitorEto> for Visitor {
}

impl From<VisitorEto> for Visitor {
    fn from(visitor_eto: VisitorEto) -> Self {
        Visitor {
            id: visitor_eto.id.unwrap(),
            modification_counter: 1,
            username: visitor_eto.username,
            name: visitor_eto.name,
            phone_number: visitor_eto.phone_number,
            password: visitor_eto.password,
            accepted_commercial: visitor_eto.accepted_commercial,
            accepted_terms: visitor_eto.accepted_terms,
            user_type: visitor_eto.user_type,
            created_at: Utc::now().naive_utc()
        }
    }
}

impl Into<VisitorEto> for Visitor {
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

