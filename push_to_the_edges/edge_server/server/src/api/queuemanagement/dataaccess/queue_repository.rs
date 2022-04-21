use diesel::{Insertable, Table};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::api::common::dataaccess::api::repo::cache_repository::CacheRepository;
use crate::api::common::logic::api::criteria::Criteria;

pub trait QueueRepository<ID, E, C, T>
    : CacheRepository<ID, E, T>
    where
        C: Criteria,
        E: Serialize + DeserializeOwned + Insertable<T>, 
        T: Table,
{}