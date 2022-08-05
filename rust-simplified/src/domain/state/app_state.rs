use crate::domain::config::dbtypes_config::DbPool;

pub struct AppState {
    pub pool: DbPool,
}

impl AppState {
    pub fn new(pool: DbPool) -> Self {
        AppState { pool }
    }
}
