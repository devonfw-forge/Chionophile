use std::env;
use serde::{Deserialize};

#[derive(Deserialize, Clone)]
pub struct ApplicationConfiguration {
    pub bind_url: String,
}

pub fn read_config() -> ApplicationConfiguration {
    let bind_url = env::var("BIND_URL").unwrap();
    ApplicationConfiguration {
        bind_url,
    }
}
