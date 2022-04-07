use diesel::{Insertable, Table};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::api::common::logic::api::criteria::Criteria;

/*
    This trait defines the operations for the UserRepository. In this case, it's enough with
    the ones provided by the general Repository trait, but if custom ones are needed they can be 
    added here.
*/
pub trait UserRepository<ID, E, NE, C, T> : Repository<ID, E, NE, C, T>
    where
        C: Criteria,
        E: Serialize + DeserializeOwned + Insertable<T>,
        T: Table,
        NE: Insertable<T> 
{}