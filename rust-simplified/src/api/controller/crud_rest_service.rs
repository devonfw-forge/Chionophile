use crate::api::model::saveable::Saveable;
use crate::domain::tos::criteria::Criteria;
use crate::AppState;
use actix_web::{web, Error, HttpResponse};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::Serialize;

#[async_trait]
pub trait CRUDRestService<ID, ETO, C, SA>
where
    C: Criteria,
    ETO: Serialize + DeserializeOwned,
    SA: Saveable,
{
    async fn search(
        app_state: web::Data<AppState>,
        criteria: web::Json<C>,
    ) -> Result<HttpResponse, Error>;

    async fn get(app_state: web::Data<AppState>, id: web::Path<ID>) -> Result<HttpResponse, Error>;

    async fn save(
        app_state: web::Data<AppState>,
        insertable: web::Json<SA>,
    ) -> Result<HttpResponse, Error>;

    async fn delete(
        app_state: web::Data<AppState>,
        id: web::Path<ID>,
    ) -> Result<HttpResponse, Error>;
}
