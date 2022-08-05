use crate::{
    api::model::visitor_eto::VisitorEto,
    domain::{
        database::schema::visitor,
        models::{entity::Entity, new_visitor::NewVisitor},
    },
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
#[serde(rename_all = "camelCase")]
#[table_name = "visitor"]
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
}

impl Entity<i64, NewVisitor, VisitorEto> for Visitor {
    fn from_insert(id: i64, new_visitor: NewVisitor) -> Self {
        Visitor {
            id,
            modification_counter: 1,
            username: new_visitor.username,
            name: new_visitor.name,
            phone_number: new_visitor.phone_number,
            password: new_visitor.password,
            accepted_commercial: new_visitor.accepted_commercial,
            accepted_terms: new_visitor.accepted_terms,
            user_type: new_visitor.user_type,
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
            user_type: self.user_type,
        }
    }
}
