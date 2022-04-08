use actix_web::{Error, web};
use async_trait::async_trait;
use crate::api::queuemanagement::logic::usecase::uc_find_queue::UcFindQueue;
use crate::api::queuemanagement::logic::usecase::uc_manage_queue::UcManageQueue;
use crate::api::queuemanagement::logic::queue_management::QueueManagement;
use crate::AppState;
use crate::core::general::search::search_result::SearchResult;
use crate::core::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;
use crate::core::queuemanagement::logic::api::queue_eto::QueueEto;
use crate::core::queuemanagement::logic::usecase::uc_find_queue_impl::UcFindQueueImpl;
use crate::core::queuemanagement::logic::usecase::uc_manage_queue_impl::UcManageQueueImpl;

pub struct QueueManagementImpl;

impl QueueManagement for QueueManagementImpl {}

#[async_trait]
impl UcManageQueue for QueueManagementImpl {
    async fn save_queue(
        app_state: web::Data<AppState>,
        queue: QueueEto
    ) -> Result<QueueEto, Error> {
        UcManageQueueImpl::save_queue(app_state, queue).await
    }

    async fn delete_queue(
        app_state: web::Data<AppState>,
        queue_id: i64
    ) -> Result<bool, Error> {
        UcManageQueueImpl::delete_queue(app_state, queue_id).await
    }

}

#[async_trait]
impl UcFindQueue for QueueManagementImpl {
    async fn find_queue(
        app_state: web::Data<AppState>,
        id: i64,
    ) -> Result<Option<QueueEto>, Error> {
        UcFindQueueImpl::find_queue(app_state, id).await
    }

    async fn find_queues(
        app_state: web::Data<AppState>,
        criteria: QueueSearchCriteria
    ) -> Result<SearchResult<QueueEto>, Error> {
        UcFindQueueImpl::find_queues(app_state, criteria).await
    }
}

