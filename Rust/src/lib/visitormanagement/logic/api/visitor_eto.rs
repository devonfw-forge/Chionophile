use serde::{Serialize, Deserialize};
use crate::lib::visitormanagement::dataacess::api::visitor::Visitor;

#[derive(Deserialize, Serialize)]
pub struct VisitorEto {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub phoneNumber: Option<String>,
    pub password: Option<String>,
    pub acceptedCommercial: Option<bool>,
    pub acceptedTerms: bool,
    pub userType: Option<bool>,
}

impl VisitorEto {

    pub fn from(visitor: Visitor) -> VisitorEto {

        VisitorEto {
            id: Some(visitor.id),
            username: visitor.username,
            name: visitor.name,
            phoneNumber: visitor.phone_number,
            password: visitor.password,
            acceptedCommercial: visitor.accepted_commercial,
            acceptedTerms: visitor.accepted_terms,
            userType: visitor.user_type
        }
    }
}
