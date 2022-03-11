use serde::{Serialize, Deserialize};
use crate::lib::visitormanagement::dataacess::api::visitor::Visitor;

#[derive(Deserialize, Serialize)]
pub struct VisitorEto {
    pub id: Option<String>,
    pub username: String,
    pub name: String,
    pub phoneNumber: String,
    pub password: Option<String>,
    pub acceptedCommercial: Option<bool>,
    pub acceptedTerms: Option<bool>,
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
