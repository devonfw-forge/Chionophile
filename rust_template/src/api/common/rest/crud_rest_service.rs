use actix_web::{Error, HttpResponse, web};
use async_trait::async_trait;
use serde::{Serialize};
use serde::de::DeserializeOwned;
use crate::api::common::logic::api::criteria::Criteria;
use crate::api::common::rest::api::saveable::Saveable;
use crate::AppState;
/*
    This is the trait that defines the operations of a Rest Service.
    The generic types are
    ID -> the ID type of the entity
    ETO  -> the ETO variant of the Entity
    C  -> The search criteria used in the method search
    SA -> The struct that will be used to save. It can be the ETO or any other object. See the Rust JTQ for a usage example
*/
#[async_trait]
pub trait CRUDRestService<ID, ETO, C, SA>
    where
        C: Criteria,
        ETO: Serialize + DeserializeOwned,
        SA: Saveable
{
    async fn search(
        app_state: web::Data<AppState>,
        criteria: web::Json<C>
    ) -> Result<HttpResponse, Error>;

    async fn get(
        app_state: web::Data<AppState>,
        id: web::Path<ID>
    ) -> Result<HttpResponse, Error>;

    async fn save(
        app_state: web::Data<AppState>,
        user: web::Json<SA>
    ) -> Result<HttpResponse, Error>;

    async fn delete(
        app_state: web::Data<AppState>,
        id: web::Path<ID>
    ) -> Result<HttpResponse, Error>;
}