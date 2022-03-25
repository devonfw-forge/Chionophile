use crate::lib::visitormanagement::logic::api::visitor_eto::VisitorEto;
use crate::schema::visitor;

#[derive(Debug, Clone, Insertable)]
#[table_name="visitor"]
pub struct NewVisitor {
    pub modification_counter: i32,
    pub username: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: bool,
    pub user_type: Option<bool>,
}
impl NewVisitor {
    pub fn from(
        visitor_eto: VisitorEto
    ) -> NewVisitor {
        NewVisitor {
            modification_counter: 1,
            username: visitor_eto.username,
            name: visitor_eto.name,
            phone_number: visitor_eto.phoneNumber,
            password: visitor_eto.password,
            accepted_commercial: visitor_eto.acceptedCommercial,
            accepted_terms: visitor_eto.acceptedTerms,
            user_type: visitor_eto.userType
        }
    }
}
