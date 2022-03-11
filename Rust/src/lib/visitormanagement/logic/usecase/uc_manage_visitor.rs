use actix_web::{Error, web};
use uuid::Uuid;
use crate::lib::general::config::db_config::DbPool;
use crate::lib::visitormanagement::dataacess::api::visitor::Visitor;
use crate::lib::visitormanagement::dataacess::api::repo::visitor_repository;
use crate::lib::visitormanagement::logic::api::visitor_eto::VisitorEto;

pub async fn save_visitor(
    pool: web::Data<DbPool>,
    visitor: VisitorEto
) -> Result<VisitorEto, Error> {
    let result = web::block(move || {
        let conn = pool.get()?;
        let new_visitor: Visitor = Visitor::from(visitor);

        visitor_repository::save(&new_visitor, &conn)
    }).await?;

    Ok(VisitorEto::from(result))
}

pub async fn delete_visitor(
    pool: web::Data<DbPool>,
    visitor_uid: Uuid
) -> Result<bool, Error> {
    web::block(move || {
        let conn = pool.get()?;
        let uuid = visitor_uid;
        visitor_repository::delete_by_id(uuid, &conn)
    }).await?;

    Ok(true)
}
