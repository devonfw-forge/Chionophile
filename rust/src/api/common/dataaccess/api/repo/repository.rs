use diesel::{Insertable, Table};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::api::common::logic::api::criteria::Criteria;
use crate::DbConn;
use crate::core::general::config::dbtypes_config::DbError;

pub trait Repository<ID, E, NE, C, T>
    where
        C: Criteria,
        E: Serialize + DeserializeOwned,
        T: Table,
        NE: Insertable<T>
{
    fn find_by_id(
        id: ID,
        conn: &DbConn
    ) -> Result<Option<E>, DbError>;

    fn find_by_criteria(
        criteria: C,
        conn: &DbConn
    ) -> Result<Vec<E>, DbError>;

    fn save(
        new_entity: &NE,
        conn: &DbConn
    ) -> Result<E, DbError>;

    fn delete_by_id(
        id: ID,
        conn: &DbConn
    ) -> Result<ID, DbError>;
}