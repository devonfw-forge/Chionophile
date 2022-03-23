use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Sort {
    property: String,
    direction: String
}
