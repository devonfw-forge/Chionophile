use diesel::{Insertable, Table};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::core::general::config::dbtypes_config::DbError;
use crate::DbConn;

pub trait CacheRepository<ID, E, T>
    where
        E: Serialize + DeserializeOwned + Insertable<T>,
        T: Table
{
    fn insert_all(
        elements: Vec<E>,
        conn: &DbConn
    ) -> Result<Vec<E>, DbError>;

    fn find_by_id(
        id: ID,
        conn: &DbConn
    ) -> Result<Option<E>, DbError>;

    fn save(
        new_entity: &E,
        conn: &DbConn
    ) -> Result<E, DbError>;

    fn delete_by_id(
        id: ID,
        conn: &DbConn
    ) -> Result<Option<ID>, DbError>;
}