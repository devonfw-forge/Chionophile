use serde::{Serialize, Deserialize};
use validator::{Validate};
use regex::Regex;
use crate::api::common::logic::api::eto::EntityETO;
use crate::api::common::rest::api::saveable::Saveable;

/*
    This struct defines the ETO for the User entity. It's the one that will be sent and received
    from the endpoints, and it validates the fields.
    The phone validator requires a country prefix to be added. An alternative validator that only
    contains number is provided commented serving both as an example for regex validation and 
    if the development teams prefers numbers without country prefix.
*/

/*lazy_static! {
    static ref PHONE_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
}*/

#[derive(Deserialize, Serialize, Validate)]
#[serde(rename_all="camelCase")]
pub struct UserEto {
    pub id: Option<i64>,
    #[validate(email(code="mail", message="Email format not valid"))]
    pub username: String,
    pub name: String,
    //#[validate(regex(path = "PHONE_REGEX", code="phone", message="Phone number format not valid"))]
    #[validate(phone(code="phone", message="Phone number format is incorrect"))]
    pub phone_number: String,
    pub password: String,
    pub accepted_commercial: Option<bool>,
    pub accepted_terms: bool,
}


impl EntityETO for UserEto {}
impl Saveable for UserEto {}