use actix_web::{Error, web};
use crate::DbError;
use crate::lib::queuemanagement::logic::api::queue_eto::QueueEto;
use crate::lib::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;
use crate::lib::queuemanagement::logic::usecase::{uc_find_queue, uc_manage_queue};
use crate::lib::general::config::db_config::DbPool;
use crate::lib::general::search_result::SearchResult;

pub async fn find_queue(
    pool: web::Data<DbPool>,
    id: i64,
) -> Result<Option<QueueEto>, Error> {
    uc_find_queue::find_queue(pool, id).await
}

pub async fn find_queues(
    pool: web::Data<DbPool>,
    criteria: QueueSearchCriteria
) -> Result<SearchResult<QueueEto>, Error> {
    uc_find_queue::find_queues(pool, criteria).await
}

pub async fn save_queue(
    pool: web::Data<DbPool>,
    queue: QueueEto
) -> Result<QueueEto, Error> {
    uc_manage_queue::save_queue(pool, queue).await
}

pub async fn delete_queue(
    pool: web::Data<DbPool>,
    queue_id: i64
) -> Result<bool, Error> {
    uc_manage_queue::delete_queue(pool, queue_id).await
}