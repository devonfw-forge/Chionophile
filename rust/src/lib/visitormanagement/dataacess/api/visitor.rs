use serde::{Serialize, Deserialize};
use crate::lib::visitormanagement::dataacess::api::new_visitor::NewVisitor;
use crate::lib::visitormanagement::logic::api::visitor_eto::VisitorEto;
use crate::schema::visitor;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
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
}

impl Visitor {
    pub fn from(
        visitor_eto: VisitorEto
    ) -> Visitor {
        let mut visitor = Visitor {
            id: -1,
            modification_counter: 1,
            username: visitor_eto.username,
            name: visitor_eto.name,
            phone_number: visitor_eto.phoneNumber,
            password: visitor_eto.password,
            accepted_commercial: visitor_eto.acceptedCommercial,
            accepted_terms: visitor_eto.acceptedTerms,
            user_type: visitor_eto.userType
        };

        if let Some(id) = visitor_eto.id {
            visitor.id = id;
        }

        visitor
    }
    pub fn from_insert(
        id: i64,
        new_visitor: NewVisitor
    ) -> Visitor {
        Visitor {
            id,
            modification_counter: 1,
            username: new_visitor.username,
            name: new_visitor.name,
            phone_number: new_visitor.phone_number,
            password: new_visitor.password,
            accepted_commercial: new_visitor.accepted_commercial,
            accepted_terms: new_visitor.accepted_terms,
            user_type: new_visitor.user_type
        }
    }
}
