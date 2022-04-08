use diesel::prelude::*;
use crate::api::common::dataaccess::api::entity::Entity;
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::api::accesscodemanagement::dataaccess::accesscode_repository::AccessCodeRepository;
use crate::DbConn;
use crate::core::general::config::dbtypes_config::{DbError, DbType};
use crate::core::accesscodemanagement::dataaccess::api::accesscode::AccessCode;
use crate::core::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::core::general::database::schema::accesscode;
use crate::core::accesscodemanagement::dataaccess::api::new_accesscode::NewAccessCode;
use crate::core::accesscodemanagement::logic::api::accesscode_cto::AccessCodeCto;
use crate::core::visitormanagement::dataaccess::api::visitor::Visitor;
use crate::core::queuemanagement::dataaccess::api::queue::Queue;

pub struct AccessCodeRepositoryImpl;

impl AccessCodeRepository<i64, AccessCode, NewAccessCode, AccessCodeSearchCriteria, accesscode::table> for AccessCodeRepositoryImpl {
    
    fn find_by_id_cto(access_code_id: i64, conn: &DbConn) -> Result<Option<AccessCodeCto>, DbError> {
        use crate::core::general::database::schema::visitor;
        use crate::core::general::database::schema::dailyqueue;

        let mut query = accesscode::table
            .inner_join(visitor::table.on(visitor::id.eq(accesscode::visitor_id)))
            .inner_join(dailyqueue::table.on(dailyqueue::id.eq(accesscode::queue_id)))
            .into_boxed::<DbType>();

        query = query.filter(accesscode::id.eq(access_code_id));

        let access_code_cto_vector: Vec<(AccessCode, Visitor, Queue)> = query.load(conn)?;
        let tuple = access_code_cto_vector.first().cloned();

        if  let Some(tuple) = tuple {
            Ok(Some(AccessCodeCto::from_query_result(tuple)))
        } else {
            Ok(None)
        }
    }

    fn find_ctos(criteria: AccessCodeSearchCriteria, conn: &DbConn) -> Result<Vec<AccessCodeCto>, DbError> {
        use crate::core::general::database::schema::visitor;
        use crate::core::general::database::schema::dailyqueue;
        
        let mut query = accesscode::table
            .inner_join(visitor::table.on(visitor::id.eq(accesscode::visitor_id)))
            .inner_join(dailyqueue::table.on(dailyqueue::id.eq(accesscode::queue_id)))
            .into_boxed::<DbType>();

        if let Some(creation_time) = criteria.creation_time {
            query = query.filter(accesscode::creation_time.eq(creation_time));
        }
        if let Some(start_time) = criteria.start_time {
            query = query.filter(accesscode::start_time.eq(start_time));
        }
        if let Some(end_time) = criteria.end_time {
            query = query.filter(accesscode::end_time.eq(end_time));
        }
        if let Some(visitor_id) = criteria.visitor_id {
            query = query.filter(accesscode::visitor_id.eq(visitor_id));
        }
        if let Some(queue_id) = criteria.queue_id {
            query = query.filter(accesscode::queue_id.eq(queue_id));
        }

        let query_results : Vec<(AccessCode, Visitor, Queue)> = query.load(conn)?;

        let paged_vector = criteria.pageable.from(query_results);

        let result_cto = paged_vector.iter().map(|res|
            AccessCodeCto::from_query_result(res.clone())
        ).collect::<Vec<AccessCodeCto>>();

        Ok(result_cto)
    }
}

impl Repository<i64, AccessCode, NewAccessCode, AccessCodeSearchCriteria, accesscode::table> for AccessCodeRepositoryImpl {
    fn find_by_id(
        accesscode_id: i64,
        conn: &DbConn
    ) -> Result<Option<AccessCode>, DbError> {
        use crate::core::general::database::schema::accesscode::dsl::*;

        let accesscode_option: Option<AccessCode> = accesscode
            .filter(id.eq(accesscode_id))
            .first::<AccessCode>(conn)
            .optional()?;

        Ok(accesscode_option)
    }

    fn find_by_criteria(
        criteria: AccessCodeSearchCriteria,
        conn: &DbConn
    ) -> Result<Vec<AccessCode>, DbError> {

        let mut query = accesscode::table.into_boxed::<DbType>();

        if let Some(creation_time) = criteria.creation_time {
            query = query.filter(accesscode::creation_time.eq(creation_time));
        }
        if let Some(start_time) = criteria.start_time {
            query = query.filter(accesscode::start_time.eq(start_time));
        }
        if let Some(end_time) = criteria.end_time {
            query = query.filter(accesscode::end_time.eq(end_time));
        }
        if let Some(visitor_id) = criteria.visitor_id {
            query = query.filter(accesscode::visitor_id.eq(visitor_id));
        }
        if let Some(queue_id) = criteria.queue_id {
            query = query.filter(accesscode::queue_id.eq(queue_id));
        }

        let query_results: Vec<AccessCode> = query.load(conn)?;

        Ok(query_results)
    }

    fn save(
        new_accesscode: &NewAccessCode,
        conn: &DbConn
    ) -> Result<AccessCode, DbError> {
        use crate::core::general::database::schema::accesscode::dsl::*;

        let accesscode_id = diesel::insert_into(accesscode)
            .values(new_accesscode)
            .returning(id)
            .get_result(conn)?;

        Ok(AccessCode::from_insert(accesscode_id, new_accesscode.clone()))
    }

    fn delete_by_id(
        accesscode_id: i64,
        conn: &DbConn
    ) -> Result<bool, DbError> {
        use crate::core::general::database::schema::accesscode::dsl::*;

        let deleted: Vec<i64> = diesel::delete(accesscode)
            .filter(id.eq(accesscode_id))
            .returning(id)
            .get_results(conn)?;

        Ok(deleted.len() > 0)
    }
}

