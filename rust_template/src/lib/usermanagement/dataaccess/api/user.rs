use crate::lib::general::database::schema::users;
use serde::{Serialize, Deserialize};
use crate::lib::usermanagement::dataaccess::api::new_user::NewUser;

/*
    This is the normal entity that will fully match with the users table defined in the schema.rs
    file. It will be used for all the read and update operations in the database.
    The serde annotation will rename all fields to camelCase when serializing the struct to JSON,
    allowing us to keep the snake_case convention in Rust code. However, this annotation is optional
    and completely depends on the conventions defined with the FrontEnd.
    All nullable fields must be wrapped in an Option<>.
*/
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, QueryableByName)]
#[serde(rename_all="camelCase")]
#[table_name="users"]
pub struct User {
    pub id: i64,
    pub username: Option<String>,
    pub name: Option<String>,
    pub password: Option<String>,
    pub phone_number: Option<String>,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: bool,
}

impl User {
    pub fn from_insert(
        id: i64,
        new_user: NewUser
    ) -> User {
        User {
            id,
            username: new_user.username,
            name: new_user.name,
            phone_number: new_user.phone_number,
            password: new_user.password,
            accepted_commercial: new_user.accepted_commercial,
            accepted_terms: new_user.accepted_terms,
        }
    }
}