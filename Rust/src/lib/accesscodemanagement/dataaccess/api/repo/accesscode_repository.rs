use diesel::prelude::*;
use uuid::Uuid;
use crate::lib::accesscodemanagement::dataaccess::api::access_code::AccessCode;
use crate::lib::accesscodemanagement::logic::api::accesscode_cto::AccessCodeCto;
use crate::lib::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::lib::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::lib::general::config::db_config::{DbError, DbType, DbConn};
use crate::lib::queuemanagement::dataacess::api::queue::Queue;
use crate::lib::visitormanagement::dataacess::api::visitor::Visitor;
use crate::schema::access_codes;

pub fn find_by_id_cto(
    id: Uuid, conn: &DbConn
) -> Result<Option<AccessCodeCto>, DbError>
{
    use crate::schema::visitors;
    use crate::schema::queues;

    let mut query = access_codes::table
        .inner_join(visitors::table.on(visitors::id.nullable().like(access_codes::visitor_id)))
        .inner_join(queues::table.on(queues::id.nullable().like(access_codes::queue_id)))
        .into_boxed::<DbType>();

    query = query.filter(access_codes::id.eq(id.to_string()));

    let access_code_cto_vector: Vec<(AccessCode, Visitor, Queue)> = query.load(conn)?;
    let tuple = access_code_cto_vector.first().cloned();

    if  let Some(tuple) = tuple {
        Ok(Some(AccessCodeCto::from_query_result(tuple)))
    } else {
        Ok(None)
    }

}

pub fn find_by_id(
    uuid: Uuid,
    conn: &DbConn
) -> Result<Option<AccessCode>, DbError> {
    use crate::schema::access_codes::dsl::*;

    let access_code: Option<AccessCode> = access_codes
        .filter(id.eq(uuid.to_string()))
        .first::<AccessCode>(conn).optional()?;

    Ok(access_code)
}

pub fn find_etos(
    filters: AccessCodeSearchCriteria,
    conn: &DbConn
) -> Result<Vec<AccessCodeEto>, DbError> {
    let params = filters.clone();
    let mut query = access_codes::table.into_boxed::<DbType>();

    if let Some(queue_id) = params.queue_id {
        query = query.filter(access_codes::queue_id.eq(queue_id));
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
    use crate::schema::visitors;
    use crate::schema::queues;

    let mut query = access_codes::table
        .inner_join(visitors::table.on(visitors::id.nullable().like(access_codes::visitor_id)))
        .inner_join(queues::table.on(queues::id.nullable().like(access_codes::queue_id)))
        .into_boxed::<DbType>();

    if let Some(ticket_number) = criteria.ticket_number {
        query = query.filter(access_codes::ticket_number.eq(ticket_number));
    }
    if let Some(creation_time) = criteria.creation_time {
        query = query.filter(access_codes::creation_time.eq(creation_time));
    }
    if let Some(start_time) = criteria.start_time {
        query = query.filter(access_codes::start_time.eq(start_time));
    }
    if let Some(end_time) = criteria.end_time {
        query = query.filter(access_codes::end_time.eq(end_time));
    }
    if let Some(visitor_id) = criteria.visitor_id {
        query = query.filter(access_codes::visitor_id.eq(visitor_id));
    }
    if let Some(queue_id) = criteria.queue_id {
        query = query.filter(access_codes::queue_id.eq(queue_id));
    }

    let query_results : Vec<(AccessCode, Visitor, Queue)> = query.load(conn)?;

    let paged_vector = criteria.pageable.from(query_results);

    let result_cto = paged_vector.iter().map(|res|
        AccessCodeCto::from_query_result(res.clone())
    ).collect::<Vec<AccessCodeCto>>();

    Ok(result_cto)
}

pub fn save(
    access_code: &AccessCode,
    conn: &DbConn
) -> Result<AccessCodeEto, DbError> {

    use crate::schema::access_codes::dsl::*;

    let mut new_access_code = access_code.clone();

    new_access_code.id = Uuid::new_v4().to_string();

    diesel::insert_into(access_codes).values(&new_access_code).execute(conn)?;

    Ok(AccessCodeEto::from(new_access_code))
}

pub fn delete(uuid: Uuid, conn: &DbConn) -> Result<Uuid, DbError> {
    use crate::schema::access_codes::dsl::*;

    diesel::delete(access_codes).filter(id.eq(uuid.to_string())).execute(conn)?;

    Ok(uuid)
}
