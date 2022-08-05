use crate::domain::database::schema::visitor;
use crate::{api::model::visitor_eto::VisitorEto, domain::models::new_entity::NewEntity};

#[derive(Debug, Clone, Insertable)]
#[table_name = "visitor"]
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

impl NewEntity<VisitorEto> for NewVisitor {}

impl From<VisitorEto> for NewVisitor {
    fn from(visitor_eto: VisitorEto) -> Self {
        NewVisitor {
            modification_counter: 1,
            username: visitor_eto.username,
            name: visitor_eto.name,
            phone_number: visitor_eto.phone_number,
            password: visitor_eto.password,
            accepted_commercial: visitor_eto.accepted_commercial,
            accepted_terms: visitor_eto.accepted_terms,
            user_type: visitor_eto.user_type,
        }
    }
}
