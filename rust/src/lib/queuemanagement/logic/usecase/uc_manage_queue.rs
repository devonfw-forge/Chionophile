use std::sync::Arc;
use actix_web::{Error, web};
use crate::{DbConn, DbError};
use crate::lib::general::config::db_config::DbPool;
use crate::lib::queuemanagement::dataacess::api::new_queue::NewQueue;
use crate::lib::queuemanagement::dataacess::api::repo::queue_repository;
use crate::lib::queuemanagement::logic::api::queue_eto::QueueEto;

pub async fn save_queue(
    pool: web::Data<DbPool>,
    queue: QueueEto
) -> Result<QueueEto, Error> {
    let result = web::block(move || {
        let conn = pool.get()?;
        let new_queue: NewQueue = NewQueue::from(queue);
        
        queue_repository::save(&new_queue, &conn)
    }).await?;

    Ok(QueueEto::from(result))
}

pub async fn delete_queue(
    pool: web::Data<DbPool>,
    queue_id: i64
) -> Result<bool, Error> {
    web::block(move || {
        let conn = pool.get()?;
        queue_repository::delete_by_id(queue_id, &conn)
    }).await?;

    Ok(true)
}
