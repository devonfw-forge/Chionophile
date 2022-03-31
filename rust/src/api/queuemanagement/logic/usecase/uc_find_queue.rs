use actix_web::{Error, web};
use async_trait::async_trait;
use crate::AppState;
use crate::core::general::search::search_result::SearchResult;
use crate::core::queuemanagement::logic::api::queue_eto::QueueEto;
use crate::core::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;

#[async_trait]
pub trait UcFindQueue {
    async fn find_queue(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<Option<QueueEto>, Error>;

    async fn find_queues(
        app_state: web::Data<AppState>,
        criteria: QueueSearchCriteria
    ) -> Result<SearchResult<QueueEto>, Error>;
}