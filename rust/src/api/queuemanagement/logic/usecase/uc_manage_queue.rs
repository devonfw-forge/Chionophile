use crate::AppState;
use actix_web::{Error, web};
use async_trait::async_trait;
use crate::core::queuemanagement::logic::api::queue_eto::QueueEto;

#[async_trait]
pub trait UcManageQueue {
    async fn save_queue(
        app_state: web::Data<AppState>,
        queue: QueueEto
    ) -> Result<QueueEto, Error>;

    async fn delete_queue(
        app_state: web::Data<AppState>,
        queue_id: i64
    ) -> Result<(), Error>;
}
