use actix_web::{Error, web};
use async_trait::async_trait;
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::api::queuemanagement::logic::usecase::uc_manage_queue::UcManageQueue;
use crate::AppState;
use crate::core::queuemanagement::dataaccess::api::new_queue::NewQueue;
use crate::core::queuemanagement::dataaccess::api::repo::queue_repository_impl::QueueRepositoryImpl;
use crate::core::queuemanagement::logic::api::queue_eto::QueueEto;

pub struct UcManageQueueImpl;

#[async_trait]
impl UcManageQueue for UcManageQueueImpl {
    async fn save_queue(
        app_state: web::Data<AppState>,
        queue: QueueEto
    ) -> Result<QueueEto, Error> {
        let result = web::block(move || {
            let conn = app_state.pool.get()?;
            let new_queue: NewQueue = NewQueue::from(queue);
            QueueRepositoryImpl::save(&new_queue, &conn)

        }).await?;

        Ok(result.into())

    }

    async fn delete_queue(
        app_state: web::Data<AppState>,
        queue_id: i64
    ) -> Result<(), Error> {
        web::block(move || {
            let conn = app_state.pool.get()?;
            QueueRepositoryImpl::delete_by_id(queue_id, &conn)
        }).await?;

        Ok(())
    }
}

