use actix_web::{Error, web};
use crate::lib::visitormanagement::logic::api::visitor_eto::VisitorEto;
use crate::lib::visitormanagement::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use crate::lib::visitormanagement::logic::usecase::{uc_find_visitor, uc_manage_visitor};
use crate::lib::general::config::db_config::DbPool;
use crate::lib::general::search_result::SearchResult;

pub async fn find_visitor(
    pool: web::Data<DbPool>,
    id: i64,
) -> Result<Option<VisitorEto>, Error> {
    uc_find_visitor::find_visitor(pool, id).await
}

pub async fn find_visitors(
    pool: web::Data<DbPool>,
    criteria: VisitorSearchCriteria
) -> Result<SearchResult<VisitorEto>, Error> {
    uc_find_visitor::find_visitors(pool, criteria).await
}

pub async fn save_visitor(
    pool: web::Data<DbPool>,
    visitor: VisitorEto
) -> Result<VisitorEto, Error> {
    uc_manage_visitor::save_visitor(pool, visitor).await
}

pub async fn delete_visitor(
    pool: web::Data<DbPool>,
    visitor_id: i64
) -> Result<bool, Error> {
    uc_manage_visitor::delete_visitor(pool, visitor_id).await
}
