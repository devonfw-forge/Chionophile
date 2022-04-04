use serde::{Serialize, Deserialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Sort {
    property: String,
    direction: String
}
