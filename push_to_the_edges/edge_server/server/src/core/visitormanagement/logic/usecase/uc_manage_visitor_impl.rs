use actix_web::{Error, web};
use async_trait::async_trait;
use crate::api::common::dataaccess::api::repo::cache_repository::CacheRepository;
use crate::api::common::errors::save_error::SaveError;
use crate::api::visitormanagement::logic::usecase::uc_manage_visitor::UcManageVisitor;
use crate::AppState;
use crate::core::visitormanagement::dataaccess::api::repo::visitor_repository_impl::VisitorRepositoryImpl;
use crate::core::visitormanagement::logic::api::visitor_eto::VisitorEto;
use crate::core::visitormanagement::dataaccess::api::visitor::Visitor;

pub struct UcManageVisitorImpl;

#[async_trait]
impl UcManageVisitor for UcManageVisitorImpl {
    async fn save_visitor(
        app_state: web::Data<AppState>,
        visitor: VisitorEto
    ) -> Result<VisitorEto, SaveError> {
        let request_url =
            format!("{}/jumpthequeue/services/rest/visitormanagement/v1/visitor/",
                    app_state.central_url);
        let client = reqwest::Client::new();

        let result = client.post(request_url).json(&visitor).send().await?;

        if result.status() != 200 {
            return Err(SaveError::InternalServerError)
        }

        let visitor: VisitorEto = result.json().await?;

        Self::cache_visitor(app_state, visitor.clone()).await?;

        Ok(visitor)
    }

    async fn cache_visitor(
        app_state: web::Data<AppState>,
        visitor: VisitorEto
    ) -> Result<VisitorEto, SaveError> {
        let result: Result<Visitor, SaveError> = web::block(move || {
            let conn = app_state.pool.get()?;
            let cached_visitor: Visitor = Visitor::from(visitor);
            let res = VisitorRepositoryImpl::save(&cached_visitor, &conn)?;

            Ok(res)
        }).await?;

        Ok(result.unwrap().into())
    }

    async fn delete_visitor(
        app_state: web::Data<AppState>,
        visitor_id: i64
    ) -> Result<Option<i64>, Error> {
        let central_url = app_state.central_url.clone();
        let deleted = web::block(move || {
            let conn = app_state.pool.get()?;
            VisitorRepositoryImpl::delete_by_id(visitor_id, &conn)
        }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

        let request_url =
            format!("{}/jumpthequeue/services/rest/visitormanagement/v1/visitor/{}/",
                    central_url, visitor_id.to_string());

        let client = reqwest::Client::new();

        client.delete(request_url).send().await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(deleted)
    }
}

