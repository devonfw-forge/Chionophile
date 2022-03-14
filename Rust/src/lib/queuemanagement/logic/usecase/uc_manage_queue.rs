use actix_web::{Error, web};
use uuid::Uuid;
use crate::lib::general::config::db_config::DbPool;
use crate::lib::queuemanagement::dataacess::api::queue::Queue;
use crate::lib::queuemanagement::dataacess::api::repo::queue_repository;
use crate::lib::queuemanagement::logic::api::queue_eto::QueueEto;

pub async fn save_queue(
    pool: web::Data<DbPool>,
    queue: QueueEto
) -> Result<QueueEto, Error> {
    let result = web::block(move || {
        let conn = pool.get()?;
        let new_queue: Queue = Queue::from(queue);

        queue_repository::save(&new_queue, &conn)
    }).await?;

    Ok(QueueEto::from(result))
}

pub async fn delete_queue(
    pool: web::Data<DbPool>,
    queue_uid: Uuid
) -> Result<bool, Error> {
    web::block(move || {
        let conn = pool.get()?;
        let uuid = queue_uid;
        queue_repository::delete_by_id(uuid, &conn)
    }).await?;

    Ok(true)
}

pub async fn decrease_queue_customer(
    pool: web::Data<DbPool>,
    queue_id: Uuid
) -> Result<(), Error> {
    web::block (move || {
        let conn = pool.get()?;
        let queue_option = queue_repository::find_by_id(queue_id, &conn)?;
        let mut queue = queue_option.unwrap();
        queue.customers = queue.customers - 1;
        let conn = pool.get()?;
        queue_repository::update_customers(&queue, &conn)
    }).await?;

    Ok(())
}
pub async fn increase_queue_customer(
    pool: web::Data<DbPool>,
    queue_id: Uuid
) -> Result<(), Error> {
    web::block (move || {
        let conn = pool.get()?;
        let queue_option = queue_repository::find_by_id(queue_id, &conn)?;
        let mut queue = queue_option.unwrap();
        queue.customers = queue.customers + 1;
        let conn = pool.get()?;
        queue_repository::update_customers(&queue, &conn)
    }).await?;
    
    Ok(())
}

