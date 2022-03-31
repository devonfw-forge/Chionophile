use serde::{Serialize, Deserialize};
use uuid::Uuid;

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
    pub fn generate_test_visitor() -> VisitorEto {
        let mail = format!("{}@mail.com", Uuid::new_v4());
        VisitorEto {
            id: None,
            username: Some(mail),
            name: Some("Goose".to_string()),
            phoneNumber: Some("123456789".to_string()),
            password: Some("1234".to_string()),
            acceptedCommercial: Some(true),
            acceptedTerms: true,
            userType: Some(true)
        }
    }
}
