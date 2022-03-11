use actix_web::{Error, web};
use chrono::{Utc};
use uuid::Uuid;
use crate::lib::general::config::db_config::DbPool;
use crate::lib::general::search_result::SearchResult;
use crate::lib::queuemanagement::logic::api::queue_eto::QueueEto;
use crate::lib::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;
use crate::lib::queuemanagement::dataacess::api::repo::queue_repository;

pub async fn find_queue(
    pool: web::Data<DbPool>,
    id: Uuid
) -> Result<Option<QueueEto>, Error> {
    let queue = web::block(move || {
        let conn = pool.get()?;
        queue_repository::find_by_id(id, &conn)
    })
        .await?;
    if let Some(queue) = queue {
        Ok(Some(QueueEto::from(queue)))
    } else {
        Ok(None)
    }
}

pub async fn find_queues(
    pool: web::Data<DbPool>,
    criteria: QueueSearchCriteria
) -> Result<SearchResult<QueueEto>, Error> {
    let pageable = criteria.pageable.clone();

    let content = web::block(move || {
        let conn = pool.get()?;
        queue_repository::find_by_criteria(criteria, &conn)
    }).await?;

    let total_elements = content.len() as i32;
    let result_content = content.iter()
        .map(|queue| QueueEto::from(queue.to_owned()))
        .collect();

    Ok(SearchResult::new(result_content, pageable, total_elements))
}

