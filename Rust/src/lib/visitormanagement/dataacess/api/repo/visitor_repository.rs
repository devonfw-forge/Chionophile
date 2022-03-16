use diesel::prelude::*;
use crate::lib::general::config::db_config::{DbError, DbConn, DbType};
use crate::lib::visitormanagement::dataacess::api::new_visitor::NewVisitor;
use crate::lib::visitormanagement::dataacess::api::visitor::Visitor;
use crate::lib::visitormanagement::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use crate::schema::visitor;

pub fn find_by_id(
    visitor_id: i64,
    conn: &DbConn
) -> Result<Option<Visitor>, DbError> {
    use crate::schema::visitor::dsl::*;

    let found_visitor: Option<Visitor> = visitor
        .filter(id.eq(visitor_id))
        .first::<Visitor>(conn)
        .optional()?;

    Ok(found_visitor)
}

pub fn save(
    new_visitor: &NewVisitor,
    conn: &DbConn
) -> Result<Visitor, DbError> {
    use crate::schema::visitor::dsl::*;

    let visitor_id = diesel::insert_into(visitor)
        .values(new_visitor)
        .returning(id)
        .get_result(conn)?;


    Ok(Visitor::from_insert(visitor_id, new_visitor.clone()))
}

pub fn find_by_criteria(
    criteria: VisitorSearchCriteria,
    conn: &DbConn
) -> Result<Vec<Visitor>, DbError>{

    let mut query = visitor::table.into_boxed::<DbType>();

    if let Some(username) = criteria.username {
        query = query.filter(visitor::username.eq(username));
    }
    if let Some(password) = criteria.password {
        query = query.filter(visitor::password.eq(password));
    }

    let results: Vec<Visitor> = query.load(conn)?;

    let paged_results = criteria.pageable.from(results);

    Ok(paged_results)
}

pub fn delete_by_id(
    visitor_id: i64,
    conn: &DbConn
) -> Result<i64, DbError> {
    use crate::schema::visitor::dsl::*;

    diesel::delete(visitor)
        .filter(id.eq(visitor_id))
        .execute(conn)?;

    Ok(visitor_id)
}
