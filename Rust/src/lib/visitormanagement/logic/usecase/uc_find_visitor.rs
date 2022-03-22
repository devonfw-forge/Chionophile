use actix_web::{Error, web};
use crate::lib::general::config::db_config::DbPool;
use crate::lib::general::search_result::SearchResult;
use crate::lib::visitormanagement::logic::api::visitor_eto::VisitorEto;
use crate::lib::visitormanagement::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use crate::lib::visitormanagement::dataacess::api::repo::visitor_repository;

pub async fn find_visitor(
    pool: web::Data<DbPool>,
    id: i64
) -> Result<Option<VisitorEto>, Error> {
    let visitor = web::block(move || {
        let conn = pool.get()?;
        visitor_repository::find_by_id(id, &conn)
    })
        .await?;

    if let Some(visitor) = visitor {
        Ok(Some(VisitorEto::from(visitor)))
    } else {
        Ok(None)
    }
}

pub async fn find_visitors(
    pool: web::Data<DbPool>,
    criteria: VisitorSearchCriteria
) -> Result<SearchResult<VisitorEto>, Error> {
    let filters = criteria.clone();
    let pageable = filters.pageable.clone();

    let content = web::block(move || {
        let conn = pool.get()?;
        visitor_repository::find_by_criteria(filters, &conn)
    }).await?;

    let total_elements = content.len() as i32;
    let result_content = content.iter()
        .map(|visitor| VisitorEto::from(visitor.to_owned()))
        .collect();

    Ok(SearchResult::new(result_content, pageable, total_elements))
}

