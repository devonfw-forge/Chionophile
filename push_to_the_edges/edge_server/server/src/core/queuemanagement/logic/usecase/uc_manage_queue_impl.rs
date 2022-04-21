use actix_web::{Error, web};
use actix_web::web::Data;
use async_trait::async_trait;
use crate::api::common::dataaccess::api::repo::cache_repository::CacheRepository;
use crate::api::common::errors::save_error::SaveError;
use crate::api::queuemanagement::logic::usecase::uc_manage_queue::UcManageQueue;
use crate::AppState;
use crate::core::queuemanagement::dataaccess::api::queue::Queue;
use crate::core::queuemanagement::dataaccess::api::repo::queue_repository_impl::QueueRepositoryImpl;
use crate::core::queuemanagement::logic::api::queue_eto::QueueEto;

pub struct UcManageQueueImpl;

#[async_trait]
impl UcManageQueue for UcManageQueueImpl {
    async fn save_queue(
        app_state: web::Data<AppState>,
        queue: QueueEto
    ) -> Result<QueueEto, SaveError> {
        let request_url =
            format!("{}/jumpthequeue/services/rest/queuemanagement/v1/queue/",
                    app_state.central_url);
        let client = reqwest::Client::new();

        let result = client.post(request_url).json(&queue).send().await?;

        if result.status() != 200 {
            return Err(SaveError::InternalServerError)
        }

        let res: QueueEto = result.json().await?;

        Self::cache_queue(app_state, res.clone()).await?;

        Ok(res)
    }

    async fn cache_queue(
        app_state: Data<AppState>,
        queue: QueueEto
    ) -> Result<QueueEto, SaveError> {
        let result = web::block(move || {
            let conn = app_state.pool.get()?;
            let cached_queue: Queue = Queue::from(queue);
            QueueRepositoryImpl::save(&cached_queue, &conn)
        }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(result.into())
    }

    async fn delete_queue(
        app_state: web::Data<AppState>,
        queue_id: i64
    ) -> Result<Option<i64>, Error> {
        let central_url = app_state.central_url.clone();
        let deleted = web::block(move || {
            let conn = app_state.pool.get()?;
            QueueRepositoryImpl::delete_by_id(queue_id, &conn)
        }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

        let request_url =
            format!("{}/jumpthequeue/services/rest/queuemanagement/v1/queue/{}/",
                    central_url, queue_id.to_string());

        let client = reqwest::Client::new();

        client.delete(request_url).send().await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(deleted)
    }
}

