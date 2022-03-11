use diesel::prelude::*;
use uuid::Uuid;

use crate::lib::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;
use crate::lib::queuemanagement::dataacess::api::queue::Queue;
use crate::lib::general::config::db_config::{DbError, DbType, DbConn};


pub fn find_by_id(
    uuid: Uuid,
    conn: &DbConn
) -> Result<Option<Queue>, DbError> {
    use crate::schema::queues::dsl::*;

    let queue: Option<Queue> = queues
        .filter(id.eq(uuid.to_string()))
        .first::<Queue>(conn).optional()?;

    Ok(queue)
}

pub fn update_customers(
    queue: &Queue,
    conn: &DbConn
) -> Result<usize, DbError> {
    use crate::schema::queues::dsl::*;

    let new_queue = queue.clone();

    let res = diesel::update(queues.filter(id.eq(&new_queue.id)))
        .set(customers.eq(&new_queue.customers))
        .execute(conn)?;

    Ok(res)
}

pub fn save(
    queue: &Queue,
    conn: &DbConn
) -> Result<Queue, DbError> {
    use crate::schema::queues::dsl::*;

    let mut new_queue = queue.clone();
    new_queue.id = Uuid::new_v4().to_string();

    diesel::insert_into(queues)
        .values(&new_queue)
        .execute(conn)?;

    Ok(new_queue)
}

pub fn delete_by_id(
    uuid: Uuid,
    conn: &DbConn
) -> Result<Uuid, DbError> {
    use crate::schema::queues::dsl::*;

    diesel::delete(queues)
        .filter(id.eq(uuid.to_string()))
        .execute(conn)?;

    Ok(uuid)
}

pub fn find_by_criteria(
    criteria: QueueSearchCriteria,
    conn: &DbConn
) -> Result<Vec<Queue>, DbError> {
    use crate::schema::queues;

    let mut query = queues::table.into_boxed::<DbType>();

    if let Some(name) = criteria.name {
        query = query.filter(queues::name.eq(name));
    }
    if let Some(logo) = criteria.logo {
        query = query.filter(queues::logo.eq(logo));
    }
    if let Some(current_number) = criteria.current_number {
        query = query.filter(queues::current_number.eq(current_number));
    }
    if let Some(attention_time) = criteria.attention_time {
        query = query.filter(queues::attention_time.eq(attention_time));
    }
    if let Some(min_attention_time) = criteria.min_attention_time {
        query = query.filter(queues::min_attention_time.eq(min_attention_time));
    }
    if let Some(active) = criteria.active {
        query = query.filter(queues::active.eq(active));
    }
    if let Some(customers) = criteria.customers {
        query = query.filter(queues::customers.eq(customers));
    }

    let results = query.load::<Queue>(conn)?;


    let paged_results = criteria.pageable.from(results);

    Ok(paged_results)
}
