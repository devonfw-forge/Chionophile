use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitorEto {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: bool,
    pub user_type: Option<bool>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VisitorPost {
    pub username: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: bool,
    pub user_type: Option<bool>,
}

impl VisitorEto {
    pub fn generate_test_get_visitor() -> VisitorEto {
        let mail = format!("{}@mail.com", Uuid::new_v4());
        VisitorEto {
            id: None,
            username: Some(mail),
            name: Some("Goose".to_string()),
            phone_number: Some("123456789".to_string()),
            password: Some("1234".to_string()),
            accepted_commercial: Some(true),
            accepted_terms: true,
            user_type: Some(true),
        }
    }
}

impl VisitorPost {
    pub fn generate_test_post_visitor() -> VisitorPost {
        let mail = format!("{}@mail.com", Uuid::new_v4());
        VisitorPost {
            username: Some(mail),
            name: Some("Goose".to_string()),
            phone_number: Some("123456789".to_string()),
            password: Some("1234".to_string()),
            accepted_commercial: Some(true),
            accepted_terms: true,
            user_type: Some(true),
        }
    }
}
