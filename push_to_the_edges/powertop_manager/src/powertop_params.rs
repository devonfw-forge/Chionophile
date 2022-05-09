use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PowerTopParams {
    pub time: String,
    pub csv: String
}