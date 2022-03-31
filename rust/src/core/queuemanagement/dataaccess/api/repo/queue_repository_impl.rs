use diesel::prelude::*;
use crate::api::common::dataaccess::api::entity::Entity;
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::api::queuemanagement::dataaccess::queue_repository::QueueRepository;
use crate::DbConn;
use crate::core::general::config::dbtypes_config::{DbError, DbType};
use crate::core::queuemanagement::dataaccess::api::queue::Queue;
use crate::core::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;
use crate::core::general::database::schema::dailyqueue;
use crate::core::queuemanagement::dataaccess::api::new_queue::NewQueue;

pub struct QueueRepositoryImpl;

impl QueueRepository<i64, Queue, NewQueue, QueueSearchCriteria, dailyqueue::table> for QueueRepositoryImpl {}

impl Repository<i64, Queue, NewQueue, QueueSearchCriteria, dailyqueue::table> for QueueRepositoryImpl {
    fn find_by_id(
        queue_id: i64,
        conn: &DbConn
    ) -> Result<Option<Queue>, DbError> {
        use crate::core::general::database::schema::dailyqueue::dsl::*;

        let queue: Option<Queue> = dailyqueue
            .filter(id.eq(queue_id))
            .first::<Queue>(conn)
            .optional()?;

        Ok(queue)
    }

    fn find_by_criteria(
        criteria: QueueSearchCriteria,
        conn: &DbConn
    ) -> Result<Vec<Queue>, DbError> {
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

        let query_results: Vec<Queue> = query.load(conn)?;


        Ok(query_results)
    }

    fn save(
        new_queue: &NewQueue,
        conn: &DbConn
    ) -> Result<Queue, DbError> {
        use crate::core::general::database::schema::dailyqueue::dsl::*;

        let queue_id = diesel::insert_into(dailyqueue)
            .values(new_queue)
            .returning(id)
            .get_result(conn)?;

        Ok(Queue::from_insert(queue_id, new_queue.clone()))
    }

    fn delete_by_id(
        queue_id: i64,
        conn: &DbConn
    ) -> Result<i64, DbError> {
        use crate::core::general::database::schema::dailyqueue::dsl::*;

        diesel::delete(dailyqueue)
            .filter(id.eq(queue_id))
            .execute(conn)?;

        Ok(queue_id)
    }
}

