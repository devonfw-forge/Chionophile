use actix_web::{Error, web};
use async_trait::async_trait;
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::api::visitormanagement::logic::usecase::uc_find_visitor::UcFindVisitor;
use crate::AppState;
use crate::core::general::search::search_result::SearchResult;
use crate::core::visitormanagement::dataaccess::api::repo::visitor_repository_impl::VisitorRepositoryImpl;
use crate::core::visitormanagement::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use crate::core::visitormanagement::logic::api::visitor_eto::VisitorEto;

pub struct UcFindVisitorImpl;

#[async_trait]
impl UcFindVisitor for UcFindVisitorImpl {
    async fn find_visitor(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<Option<VisitorEto>, Error> {
        let visitor = web::block(move || {
            let conn = app_state.pool.get()?;
            VisitorRepositoryImpl::find_by_id(id, &conn)
        })
            .await?;

        if let Some(visitor) = visitor {
            Ok(Some(visitor.into()))
        } else {
            Ok(None)
        }
    }

    async fn find_visitors(
        app_state: web::Data<AppState>,
        criteria: VisitorSearchCriteria
    ) -> Result<SearchResult<VisitorEto>, Error> {
        let filters = criteria.clone();
        let pageable = filters.pageable.clone();

        let query_results = web::block(move || {
            let conn = app_state.pool.get()?;
            VisitorRepositoryImpl::find_by_criteria(filters, &conn)
        }).await?;

        let total_elements = query_results.len() as i32;
        let paged_results = criteria.pageable.from(query_results);
        let result_content = paged_results.iter()
            .map(|visitor| visitor.clone().into())
            .collect();

        Ok(SearchResult::new(result_content, pageable, total_elements))
    }
}

