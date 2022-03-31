use actix_web::{Error, web};
use async_trait::async_trait;
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::api::queuemanagement::logic::usecase::uc_find_queue::UcFindQueue;
use crate::AppState;
use crate::core::general::search::search_result::SearchResult;
use crate::core::queuemanagement::dataaccess::api::repo::queue_repository_impl::QueueRepositoryImpl;
use crate::core::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;
use crate::core::queuemanagement::logic::api::queue_eto::QueueEto;

pub struct UcFindQueueImpl;

#[async_trait]
impl UcFindQueue for UcFindQueueImpl {
    async fn find_queue(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<Option<QueueEto>, Error> {
        let queue = web::block(move || {
            let conn = app_state.pool.get()?;
            QueueRepositoryImpl::find_by_id(id, &conn)
        })
            .await?;

        if let Some(queue) = queue {
            Ok(Some(queue.into()))
        } else {
            Ok(None)
        }
    }

    async fn find_queues(
        app_state: web::Data<AppState>,
        criteria: QueueSearchCriteria
    ) -> Result<SearchResult<QueueEto>, Error> {
        let filters = criteria.clone();
        let pageable = filters.pageable.clone();

        let query_results = web::block(move || {
            let conn = app_state.pool.get()?;
            QueueRepositoryImpl::find_by_criteria(filters, &conn)
        }).await?;

        let total_elements = query_results.len() as i32;
        let paged_results = criteria.pageable.from(query_results);
        let result_content = paged_results.iter()
            .map(|queue| queue.clone().into())
            .collect();

        Ok(SearchResult::new(result_content, pageable, total_elements))
    }
}

