use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct PowerTopParams {
    pub workload: String,
    pub csv: String
}