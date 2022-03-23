use diesel::prelude::*;
use crate::lib::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;
use crate::lib::queuemanagement::dataacess::api::queue::Queue;
use crate::lib::general::config::db_config::{DbError, DbType, DbConn};
use crate::lib::queuemanagement::dataacess::api::new_queue::NewQueue;


pub fn find_by_id(
    queue_id: i64,
    conn: &DbConn
) -> Result<Option<Queue>, DbError> {
    use crate::schema::dailyqueue::dsl::*;

    let queue: Option<Queue> = dailyqueue
        .filter(id.eq(queue_id))
        .first::<Queue>(conn).optional()?;

    Ok(queue)
}

pub fn update_customers(
    queue: &Queue,
    conn: &DbConn
) -> Result<usize, DbError> {
    use crate::schema::dailyqueue::dsl::*;

    let new_queue = queue.clone();

    let res = diesel::update(dailyqueue.filter(id.eq(&new_queue.id)))
        .set(customers.eq(&new_queue.customers))
        .execute(conn)?;

    Ok(res)
}

pub fn save(
    queue: &NewQueue,
    conn: &DbConn
) -> Result<Queue, DbError> {
    use crate::schema::dailyqueue::dsl::*;

    let new_queue_id = diesel::insert_into(dailyqueue)
        .values(queue)
        .returning(id)
        .get_result(conn)?;

    Ok(Queue::from_insert(new_queue_id, queue.clone()))
}

pub fn delete_by_id(
    queue_id: i64,
    conn: &DbConn
) -> Result<i64, DbError> {
    use crate::schema::dailyqueue::dsl::*;

    diesel::delete(dailyqueue)
        .filter(id.eq(queue_id))
        .execute(conn)?;

    Ok(queue_id)
}

pub fn find_by_criteria(
    criteria: QueueSearchCriteria,
    conn: &DbConn
) -> Result<Vec<Queue>, DbError> {
    use crate::schema::dailyqueue;

    let mut query = dailyqueue::table.into_boxed::<DbType>();

    if let Some(name) = criteria.name {
        query = query.filter(dailyqueue::name.eq(name));
    }
    if let Some(logo) = criteria.logo {
        query = query.filter(dailyqueue::logo.eq(logo));
    }
    if let Some(current_number) = criteria.current_number {
        query = query.filter(dailyqueue::current_number.eq(current_number));
    }
    if let Some(attention_time) = criteria.attention_time {
        query = query.filter(dailyqueue::attention_time.eq(attention_time));
    }
    if let Some(min_attention_time) = criteria.min_attention_time {
        query = query.filter(dailyqueue::min_attention_time.eq(min_attention_time));
    }
    if let Some(active) = criteria.active {
        query = query.filter(dailyqueue::active.eq(active));
    }
    if let Some(customers) = criteria.customers {
        query = query.filter(dailyqueue::customers.eq(customers));
    }

    let results = query.load::<Queue>(conn)?;


    let paged_results = criteria.pageable.from(results);

    Ok(paged_results)
}
