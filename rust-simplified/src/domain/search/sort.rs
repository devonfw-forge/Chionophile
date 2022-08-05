use serde:: {Serialize, Deserialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Sort {
    property: String,
    direction: String
}
