use crate::{CacheService};
use crate::api::cache_error::CacheError;
use crate::api::cache_repository::CacheRepository;
use crate::general::config::dbtypes_config::DbPool;

pub struct CacheServiceImpl;

impl CacheService for CacheServiceImpl {
    fn delete_cache<R>(
        pool: DbPool, 
    ) -> Result<(), CacheError> where R: CacheRepository {
        let conn = pool.get()?;
        R::delete_cache(&conn)?;

        Ok(())
    }
}