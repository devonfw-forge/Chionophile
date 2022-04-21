use crate::DbConn;
use crate::general::config::dbtypes_config::DbError;

pub trait CacheRepository {
    fn delete_cache(
        conn: &DbConn,
    ) -> Result<(), DbError>;
}