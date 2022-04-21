use actix_web::{Error, web};
use async_trait::async_trait;
use crate::api::common::dataaccess::api::repo::cache_repository::CacheRepository;
use crate::api::visitormanagement::logic::usecase::uc_find_visitor::UcFindVisitor;
use crate::api::visitormanagement::logic::usecase::uc_manage_visitor::UcManageVisitor;
use crate::AppState;
use crate::core::general::search::search_result::SearchResult;
use crate::core::visitormanagement::dataaccess::api::repo::visitor_repository_impl::VisitorRepositoryImpl;
use crate::core::visitormanagement::dataaccess::api::visitor::Visitor;
use crate::core::visitormanagement::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use crate::core::visitormanagement::logic::api::visitor_eto::VisitorEto;
use crate::core::visitormanagement::logic::usecase::uc_manage_visitor_impl::UcManageVisitorImpl;

pub struct UcFindVisitorImpl;

#[async_trait]
impl UcFindVisitor for UcFindVisitorImpl {
    async fn find_visitor(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<Option<VisitorEto>, Error> {
        let request_state = app_state.clone();

        let visitor = web::block(move || {
            let conn = app_state.pool.get()?;
            VisitorRepositoryImpl::find_by_id(id, &conn)
        }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

        if let Some(visitor) = visitor {
            return Ok(Some(visitor.into()))
        }
        println!("No visitor in edge");
        let request_url =
            format!("{}/jumpthequeue/services/rest/visitormanagement/v1/visitor/{}/",
                    &request_state.central_url, id);

        let response = reqwest::get(&request_url).await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        if response.status() == 404 {
            println!("No visitor in central");
            return Ok(None)
        } else {
            let visitor: VisitorEto = response.json().await
                .map_err(actix_web::error::ErrorInternalServerError)?;

            UcManageVisitorImpl::cache_visitor(request_state, visitor.clone()).await
                .map_err(actix_web::error::ErrorInternalServerError)?;

            Ok(Some(visitor))
        }

    }

    async fn find_visitors(
        app_state: web::Data<AppState>,
        criteria: VisitorSearchCriteria
    ) -> Result<SearchResult<VisitorEto>, Error> {
        let request_url =
            format!("{}/jumpthequeue/services/rest/visitormanagement/v1/visitor/search/",
                    app_state.central_url);

        let client = reqwest::Client::new();
        let response = client.post(request_url).json(&criteria).send().await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        let search_results: SearchResult<VisitorEto> = response.json().await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let cached_visitors: Vec<Visitor> = search_results.content
            .iter()
            .map(|visitor_eto| Visitor::from(visitor_eto.clone()))
            .collect::<Vec<Visitor>>();

        let _inserted_visitors = web::block(move || {
            let conn = app_state.pool.get()?;
            VisitorRepositoryImpl::insert_all(cached_visitors, &conn)
        }).await.map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(search_results)
    }
}