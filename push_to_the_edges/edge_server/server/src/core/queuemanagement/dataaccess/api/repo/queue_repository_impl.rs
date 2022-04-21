use diesel::pg::upsert::excluded;
use diesel::prelude::*;
use crate::api::common::dataaccess::api::repo::cache_repository::CacheRepository;
use crate::api::queuemanagement::dataaccess::queue_repository::QueueRepository;
use crate::DbConn;
use crate::core::general::config::dbtypes_config::{DbError};
use crate::core::queuemanagement::dataaccess::api::queue::Queue;
use crate::core::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;
use crate::core::general::database::schema::dailyqueue;
use crate::core::general::database::schema::dailyqueue::table;

pub struct QueueRepositoryImpl;

impl QueueRepository<i64, Queue, QueueSearchCriteria, dailyqueue::table> for QueueRepositoryImpl {}

impl CacheRepository<i64, Queue, table> for QueueRepositoryImpl {
    fn insert_all(
        queues: Vec<Queue>,
        conn: &DbConn
    ) -> Result<Vec<Queue>, DbError> {
        use crate::core::general::database::schema::dailyqueue::dsl::dailyqueue;
        use crate::core::general::database::schema::dailyqueue::*;

        let inserted = diesel::insert_into(dailyqueue)
            .values(queues)
            .on_conflict(id)
            .do_update()
            .set((
                name.eq(excluded(name)),
                logo.eq(excluded(logo)),
                current_number.eq(excluded(current_number)),
                attention_time.eq(excluded(attention_time)),
                min_attention_time.eq(excluded(min_attention_time)),
                active.eq(excluded(active)),
                created_at.eq(excluded(created_at))
            ))
            .get_results(conn)?;

        Ok(inserted)
    }

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

    fn save(
        cached_queue: &Queue,
        conn: &DbConn
    ) -> Result<Queue, DbError> {
        use crate::core::general::database::schema::dailyqueue::dsl::*;

        diesel::insert_into(dailyqueue)
            .values(cached_queue)
            .on_conflict(id)
            .do_update()
            .set(cached_queue)
            .execute(conn)?;

        Ok(cached_queue.clone())
    }

    fn delete_by_id(
        queue_id: i64,
        conn: &DbConn
    ) -> Result<Option<i64>, DbError> {
        use crate::core::general::database::schema::dailyqueue::dsl::*;

        let deleted: Option<i64> = diesel::delete(dailyqueue)
            .filter(id.eq(queue_id))
            .returning(id)
            .get_result(conn)
            .optional()?;

        Ok(deleted)
    }
}