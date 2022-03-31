use crate::core::general::config::dbtypes_config::DbPool;

/*
    There are variables that need to be accessed by all threads and in different points of the
    application, such as the DB connection pool or some critical section. This struct provides access
    to that information.
*/
pub struct AppState {
    pub pool: DbPool
}

impl AppState {
    pub fn new(pool: DbPool) -> Self {
        AppState {
            pool
        }
    }
}