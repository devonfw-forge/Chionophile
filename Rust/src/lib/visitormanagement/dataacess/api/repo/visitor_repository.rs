use diesel::prelude::*;
use uuid::Uuid;
use crate::lib::general::config::db_config::{DbError, DbConn, DbType};
use crate::lib::visitormanagement::dataacess::api::visitor::Visitor;
use crate::lib::visitormanagement::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use crate::schema::visitors;

pub fn find_by_id(
    uuid: Uuid,
    conn: &DbConn
) -> Result<Option<Visitor>, DbError> {
    use crate::schema::visitors::dsl::*;

    let visitor: Option<Visitor> = visitors
        .filter(id.eq(uuid.to_string()))
        .first::<Visitor>(conn)
        .optional()?;

    Ok(visitor)
}

pub fn save(
    visitor: &Visitor,
    conn: &DbConn
) -> Result<Visitor, DbError> {
    use crate::schema::visitors::dsl::*;

    let mut new_visitor = visitor.clone();

    new_visitor.id = Uuid::new_v4().to_string();

    diesel::insert_into(visitors)
        .values(&new_visitor)
        .execute(conn)?;

    Ok(new_visitor)
}

pub fn find_by_criteria(
    criteria: VisitorSearchCriteria,
    conn: &DbConn
) -> Result<Vec<Visitor>, DbError>{

    let mut query = visitors::table.into_boxed::<DbType>();

    if let Some(username) = criteria.username {
        query = query.filter(visitors::username.eq(username));
    }
    if let Some(password) = criteria.password {
        query = query.filter(visitors::password.eq(password));
    }

    let results: Vec<Visitor> = query.load(conn)?;

    let paged_results = criteria.pageable.from(results);

    Ok(paged_results)
}

pub fn delete_by_id(
    uuid: Uuid,
    conn: &DbConn
) -> Result<Uuid, DbError> {
    use crate::schema::visitors::dsl::*;

    diesel::delete(visitors)
        .filter(id.eq(uuid.to_string()))
        .execute(conn)?;

    Ok(uuid)
}
