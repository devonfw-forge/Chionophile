use diesel::prelude::*;
use crate::lib::accesscodemanagement::dataaccess::api::new_access_code::NewAccessCode;
use crate::lib::accesscodemanagement::dataaccess::api::access_code::AccessCode;
use crate::lib::accesscodemanagement::logic::api::accesscode_cto::AccessCodeCto;
use crate::lib::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::lib::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::lib::general::config::db_config::{DbError, DbType, DbConn};
use crate::lib::queuemanagement::dataacess::api::queue::Queue;
use crate::lib::visitormanagement::dataacess::api::visitor::Visitor;
use crate::schema::accesscode;

pub fn find_by_id_cto(
    access_code_id: i64,
    conn: &DbConn
) -> Result<Option<AccessCodeCto>, DbError>
{
    use crate::schema::visitor;
    use crate::schema::dailyqueue;

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

pub fn find_by_id(
    access_code_id: i64,
    conn: &DbConn
) -> Result<Option<AccessCode>, DbError> {
    use crate::schema::accesscode::dsl::*;

    let access_code: Option<AccessCode> = accesscode
        .filter(id.eq(access_code_id))
        .first::<AccessCode>(conn).optional()?;

    Ok(access_code)
}

pub fn find_etos(
    filters: AccessCodeSearchCriteria,
    conn: &DbConn
) -> Result<Vec<AccessCodeEto>, DbError> {
    let params = filters.clone();
    let mut query = accesscode::table.into_boxed::<DbType>();

    if let Some(queue_id) = params.queue_id {
        query = query.filter(accesscode::queue_id.eq(queue_id));
    }

    let query_result: Vec<AccessCode> = query.load(conn)?;

    let result = query_result.iter().map(|access_code| {
        AccessCodeEto::from(access_code.clone())
    }).collect::<Vec<AccessCodeEto>>();

    Ok(result)
}

pub fn find_ctos(
    criteria: AccessCodeSearchCriteria,
    conn: &DbConn
) -> Result<Vec<AccessCodeCto>, DbError> {
    use crate::schema::visitor;
    use crate::schema::dailyqueue;

    let mut query = accesscode::table
        .inner_join(visitor::table.on(visitor::id.eq(accesscode::visitor_id)))
        .inner_join(dailyqueue::table.on(dailyqueue::id.eq(accesscode::queue_id)))
        .into_boxed::<DbType>();

    if let Some(ticket_number) = criteria.ticket_number {
        query = query.filter(accesscode::ticket_number.eq(ticket_number));
    }
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

pub fn save(
    access_code: &NewAccessCode,
    conn: &DbConn
) -> Result<AccessCodeEto, DbError> {

    use crate::schema::accesscode::dsl::*;

    let access_code_id = diesel::insert_into(accesscode)
        .values(access_code)
        .returning(id)
        .get_result(conn)?;

    Ok(AccessCodeEto::from_insert(access_code_id, access_code.clone()))
}

pub fn delete(access_code_id: i64, conn: &DbConn) -> Result<i64, DbError> {
    use crate::schema::accesscode::dsl::*;

    diesel::delete(accesscode).filter(id.eq(access_code_id)).execute(conn)?;

    Ok(access_code_id)
}
