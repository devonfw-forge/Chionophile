use serde::{Deserialize};

#[derive(Deserialize, Clone)]
pub struct ApplicationConfiguration {
    pub bind_url: String,
}

pub fn read_config() -> ApplicationConfiguration {
    let bind_url = "0.0.0.0:8083".to_string();
    ApplicationConfiguration {
        bind_url,
    }
}
