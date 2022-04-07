use diesel::{Insertable, Table};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::api::common::logic::api::criteria::Criteria;
use crate::DbConn;
use crate::core::general::config::dbtypes_config::DbError;

/*
    This trait defines the functions for a CRUD Respository. This trait is not absolutely needed
    but it streamlines the creation of simple endpoints for a given entity.
    The generic types are
    ID -> the ID type of the entity
    E  -> the entity
    NE -> An entity without ID field. It is necessary because IDs should not be nullable in the 
          database schema, and with Diesel we can't put the Entity's ID as nullable because it has
          to match the DB table.
    C  -> The search criteria used in the method search
*/
pub trait Repository<ID, E, NE, C, T>
    where
        C: Criteria,
        E: Serialize + DeserializeOwned + Insertable<T>,
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

    fn update(
        entity: &E,
        conn: &DbConn
    ) -> Result<E, DbError>;

    fn delete_by_id(
        id: ID,
        conn: &DbConn
    ) -> Result<ID, DbError>;
}