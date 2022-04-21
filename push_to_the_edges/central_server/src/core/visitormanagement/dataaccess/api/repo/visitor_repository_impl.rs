use diesel::dsl::IntoBoxed;
use diesel::prelude::*;
use crate::api::common::dataaccess::api::entity::Entity;
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::api::visitormanagement::dataaccess::visitor_repository::VisitorRepository;
use crate::DbConn;
use crate::core::general::config::dbtypes_config::{DbError, DbType};
use crate::core::visitormanagement::dataaccess::api::visitor::Visitor;
use crate::core::visitormanagement::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use crate::core::general::database::schema::visitor;
use crate::core::visitormanagement::dataaccess::api::new_visitor::NewVisitor;

pub struct VisitorRepositoryImpl;

impl VisitorRepository<i64, Visitor, NewVisitor, VisitorSearchCriteria, visitor::table> for VisitorRepositoryImpl {}

impl Repository<i64, Visitor, NewVisitor, VisitorSearchCriteria, visitor::table> for VisitorRepositoryImpl {
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

    fn find_by_criteria(
        criteria: VisitorSearchCriteria,
        conn: &DbConn
    ) -> Result<Vec<Visitor>, DbError> {

        let mut query: IntoBoxed<visitor::table, DbType> = visitor::table.into_boxed::<DbType>();

        if let Some(username) = criteria.username {
            query = query.filter(visitor::username.eq(username));
        }
        if let Some(name) = criteria.name {
            query = query.filter(visitor::name.eq(name));
        }
        if let Some(phone_number) = criteria.phone_number {
            query = query.filter(visitor::phone_number.eq(phone_number));
        }
        if let Some(password) = criteria.password {
            query = query.filter(visitor::password.eq(password));
        }
        if let Some(accepted_commercial) = criteria.accepted_commercial {
            query = query.filter(visitor::accepted_commercial.eq(accepted_commercial));
        }
        if let Some(accepted_terms) = criteria.accepted_terms {
            query = query.filter(visitor::accepted_terms.eq(accepted_terms));
        }
        if let Some(user_type) = criteria.user_type {
            query = query.filter(visitor::user_type.eq(user_type));
        }

        let query_results: Vec<Visitor> = query.load(conn)?;


        Ok(query_results)
    }

    fn save(
        new_visitor: &NewVisitor,
        conn: &DbConn
    ) -> Result<Visitor, DbError> {
        use crate::core::general::database::schema::visitor::dsl::*;

        let visitor_id = diesel::insert_into(visitor)
            .values(new_visitor)
            .returning(id)
            .get_result(conn)?;

        Ok(Visitor::from_insert(visitor_id, new_visitor.clone()))
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

