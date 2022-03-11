
use actix_web::{Error, web};
use uuid::Uuid;
use crate::lib::accesscodemanagement::dataaccess::api::repo::accesscode_repository;
use crate::lib::accesscodemanagement::logic::api::accesscode_cto::AccessCodeCto;
use crate::lib::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::lib::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::lib::general::config::db_config::DbPool;
use crate::lib::general::search_result::SearchResult;

pub async fn find_accesscode_cto(
    pool: web::Data<DbPool>,
    id: Uuid
) -> Result<Option<AccessCodeCto>, Error> {
    let access_code = web::block(move || {
        let conn = pool.get()?;
        accesscode_repository::find_by_id_cto(id, &conn)
    })
        .await?;

    Ok(access_code)
}

pub async fn find_accesscode_ctos(
    pool: web::Data<DbPool>,
    criteria: AccessCodeSearchCriteria
) -> Result<SearchResult<AccessCodeCto>, Error> {
    let filters = criteria.clone();
    let pageable = filters.pageable.clone();

    let content = web::block(move || {
        let conn = pool.get()?;
        accesscode_repository::find_ctos(filters, &conn)
    }).await?;

    let total_elements = content.len() as i32;
    let search_results = SearchResult::new(content, pageable, total_elements);

    Ok(search_results)
}

pub async fn find_accesscode_etos(
    pool: web::Data<DbPool>,
    criteria: AccessCodeSearchCriteria
) -> Result<SearchResult<AccessCodeEto>, Error> {
    let filters = criteria.clone();
    let pageable = filters.pageable.clone();

    let content = web::block(move || {
        let conn = pool.get()?;
        accesscode_repository::find_etos(filters, &conn)
    }).await?;
    
    let total_elements = content.len() as i32;
    let search_results = SearchResult::new(content, pageable, total_elements);

    Ok(search_results)
}


