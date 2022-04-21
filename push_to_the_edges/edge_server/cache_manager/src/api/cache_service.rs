use crate::api::cache_error::CacheError;
use crate::api::cache_repository::CacheRepository;
use crate::general::config::dbtypes_config::DbPool;

pub trait CacheService {
    fn delete_cache<R>(
        pool: DbPool
    ) -> Result<(), CacheError>
    where R: CacheRepository;
}