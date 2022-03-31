use std::env;
use serde::{Deserialize};

/*
    This structure provides a way to read and store the environment variables present in the
    .cargo/config files, to make it readable in the main function.
*/
#[derive(Deserialize, Clone)]
pub struct ApplicationConfiguration {
    pub database_url: String,
    pub bind_url: String,
    pub base_rest_url: String,
}

pub fn read_config() -> ApplicationConfiguration {
    let database_url = env::var("DATABASE_URL").unwrap();
    let bind_url = env::var("BIND_URL").unwrap();
    let base_rest_url = env::var("BASE_REST_URL").unwrap();
    ApplicationConfiguration {
        database_url,
        bind_url,
        base_rest_url
    }
}
