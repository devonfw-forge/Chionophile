use actix_web::{Error, web};
use crate::lib::general::config::db_config::DbPool;
use crate::lib::visitormanagement::dataacess::api::new_visitor::NewVisitor;
use crate::lib::visitormanagement::dataacess::api::repo::visitor_repository;
use crate::lib::visitormanagement::logic::api::visitor_eto::VisitorEto;

pub async fn save_visitor(
    pool: web::Data<DbPool>,
    visitor: VisitorEto
) -> Result<VisitorEto, Error> {
    let result = web::block(move || {
        let conn = pool.get()?;
        let new_visitor: NewVisitor = NewVisitor::from(visitor);

        visitor_repository::save(&new_visitor, &conn)
    }).await?;

    Ok(VisitorEto::from(result))
}

pub async fn delete_visitor(
    pool: web::Data<DbPool>,
    visitor_id: i64
) -> Result<bool, Error> {
    web::block(move || {
        let conn = pool.get()?;
        visitor_repository::delete_by_id(visitor_id, &conn)
    }).await?;

    Ok(true)
}
