use chrono::{Duration, Utc};
use crate::DbConn;
use diesel::prelude::*;
use crate::api::cache_repository::CacheRepository;
use crate::general::config::dbtypes_config::DbError;
use crate::general::database::schema::accesscode::dsl::*;

pub struct AccessCodeRepository;

impl CacheRepository for AccessCodeRepository {
    fn delete_cache(conn: &DbConn) -> Result<(), DbError> {
        let ten_minutes = (Utc::now() - Duration::minutes(10)).naive_utc();

        diesel::delete(accesscode)
            .filter(created_at.lt(ten_minutes))
            .execute(conn)?;

        Ok(())
    }
}