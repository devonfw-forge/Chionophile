use crate::core::general::config::dbtypes_config::DbPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: DbPool,
    pub central_url: String
}

impl AppState {
    pub fn new(pool: &DbPool, central_url: &String) -> Self {
        AppState {
            pool: pool.clone(),
            central_url: central_url.to_owned()
        }
    }
}