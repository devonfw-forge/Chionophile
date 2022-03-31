use diesel::{Insertable, Table};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::api::common::logic::api::criteria::Criteria;

pub trait VisitorRepository<ID, E, NE, C, T> : Repository<ID, E, NE, C, T>
    where
        C: Criteria,
        E: Serialize + DeserializeOwned,
        T: Table,
        NE: Insertable<T>
{}