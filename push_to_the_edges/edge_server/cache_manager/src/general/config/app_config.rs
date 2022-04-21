use std::env;
use serde::{Deserialize};

#[derive(Deserialize, Clone)]
pub struct ApplicationConfiguration {
    pub database_url: String,
}

impl ApplicationConfiguration {
    pub fn init() -> Self {
        let database_url = env::var("DATABASE_URL").unwrap();
        ApplicationConfiguration {
            database_url,
        }
    }
}