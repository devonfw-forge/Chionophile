use diesel::r2d2;
use crate::general::config::dbtypes_config::DbError;

#[derive(Debug)]
pub enum CacheError {
    PoolError(r2d2::PoolError),
    DbError(DbError)
}

impl From<r2d2::PoolError> for CacheError {
    fn from(error: r2d2::PoolError) -> Self {
        CacheError::PoolError(error)
    }
}

impl From<DbError> for CacheError {
    fn from(error: DbError) -> Self {
        CacheError::DbError(error)
    }
}