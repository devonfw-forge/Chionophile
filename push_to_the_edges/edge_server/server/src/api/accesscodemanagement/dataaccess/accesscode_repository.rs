use diesel::{Insertable, Table};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::api::common::dataaccess::api::repo::cache_repository::CacheRepository;
use crate::api::common::logic::api::criteria::Criteria;
use crate::core::accesscodemanagement::logic::api::accesscode_cto::AccessCodeCto;
use crate::core::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::core::general::config::dbtypes_config::DbError;
use crate::DbConn;

pub trait AccessCodeRepository<ID, E, C, T>
    : CacheRepository<ID, E, T>
    where
        C: Criteria,
        E: Serialize + DeserializeOwned + Insertable<T>,
        T: Table,
{
    fn find_by_id_cto(
        access_code_id: i64,
        conn: &DbConn
    ) -> Result<Option<AccessCodeCto>, DbError>;

    fn find_ctos(
        criteria: AccessCodeSearchCriteria,
        conn: &DbConn
    ) -> Result<Vec<AccessCodeCto>, DbError>;
}