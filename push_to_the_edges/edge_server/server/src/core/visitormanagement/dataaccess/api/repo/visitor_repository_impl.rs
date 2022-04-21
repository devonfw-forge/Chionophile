use diesel::pg::upsert::excluded;
use diesel::prelude::*;
use crate::api::common::dataaccess::api::repo::cache_repository::CacheRepository;
use crate::api::visitormanagement::dataaccess::visitor_repository::VisitorRepository;
use crate::DbConn;
use crate::core::general::config::dbtypes_config::{DbError};
use crate::core::visitormanagement::dataaccess::api::visitor::Visitor;
use crate::core::visitormanagement::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use crate::core::general::database::schema::visitor;
use crate::core::general::database::schema::visitor::table;

pub struct VisitorRepositoryImpl;

impl VisitorRepository<i64, Visitor, VisitorSearchCriteria, visitor::table> for VisitorRepositoryImpl {}

impl CacheRepository<i64, Visitor, table> for VisitorRepositoryImpl {
    fn insert_all(
        visitors: Vec<Visitor>,
        conn: &DbConn
    ) -> Result<Vec<Visitor>, DbError> {
        use crate::core::general::database::schema::visitor::dsl::visitor;
        use crate::core::general::database::schema::visitor::*;

        let inserted = diesel::insert_into(visitor)
            .values(visitors)
            .on_conflict(id)
            .do_update()
            .set((
                name.eq(excluded(name)),
                username.eq(excluded(username)),
                password.eq(excluded(password)),
                phone_number.eq(excluded(phone_number)),
                accepted_commercial.eq(excluded(accepted_commercial)),
                accepted_terms.eq(excluded(accepted_terms)),
                user_type.eq(excluded(user_type)),
                created_at.eq(excluded(created_at))
            ))
            .get_results(conn)?;

        Ok(inserted)
    }

    fn find_by_id(
        visitor_id: i64,
        conn: &DbConn
    ) -> Result<Option<Visitor>, DbError> {
        use crate::core::general::database::schema::visitor::dsl::*;

        let visitor_option: Option<Visitor> = visitor
            .filter(id.eq(visitor_id))
            .first::<Visitor>(conn)
            .optional()?;

        Ok(visitor_option)
    }

    fn save(
        cached_visitor: &Visitor,
        conn: &DbConn
    ) -> Result<Visitor, DbError> {
        use crate::core::general::database::schema::visitor::dsl::*;

        diesel::insert_into(visitor)
            .values(cached_visitor)
            .on_conflict(id)
            .do_update()
            .set(cached_visitor)
            .execute(conn)?;

        Ok(cached_visitor.clone())
    }

    fn delete_by_id(
        visitor_id: i64,
        conn: &DbConn
    ) -> Result<Option<i64>, DbError> {
        use crate::core::general::database::schema::visitor::dsl::*;

        let deleted_visitors: Option<i64> = diesel::delete(visitor)
            .filter(id.eq(visitor_id))
            .returning(id)
            .get_result(conn)
            .optional()?;

        Ok(deleted_visitors)
    }
}