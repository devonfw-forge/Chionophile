use serde::{Serialize, Deserialize};
use crate::lib::usermanagement::dataaccess::api::user::User;

#[derive(Deserialize, Serialize)]
#[serde(rename_all="camelCase")]
pub struct UserEto {
    pub id: Option<i64>,
    pub username: Option<String>,
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub password: Option<String>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: bool,
}

impl UserEto {
    pub fn from(user: User) -> UserEto {
        UserEto {
            id: Some(user.id),
            username: user.username,
            name: user.name,
            phone_number: user.phone_number,
            password: user.password,
            accepted_commercial: user.accepted_commercial,
            accepted_terms: user.accepted_terms,
        }
    }
}
