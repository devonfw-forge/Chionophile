use serde::{Serialize, Deserialize};
use crate::lib::visitormanagement::logic::api::visitor_eto::VisitorEto;
use crate::schema::visitors;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Identifiable, QueryableByName)]
#[serde(rename_all = "camelCase")]
#[table_name="visitors"]
pub struct Visitor {
    pub id: String,
    pub username: String,
    pub name: String,
    pub phone_number: String,
    pub password: Option<String>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: Option<bool>,
    pub user_type: Option<bool>,
}

impl Visitor {
    pub fn from(
        visitor_eto: VisitorEto
    ) -> Visitor {
        let mut visitor = Visitor {
            id: "".to_string(),
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
}
