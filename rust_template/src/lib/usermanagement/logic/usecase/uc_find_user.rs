use actix_web::{Error, web};
use crate::lib::general::config::dbtypes_config::DbPool;
use crate::lib::general::search::search_result::SearchResult;
use crate::lib::usermanagement::dataaccess::api::repo::user_repository;
use crate::lib::usermanagement::logic::api::user_search_criteria::UserSearchCriteria;
use crate::lib::usermanagement::logic::api::user_eto::UserEto;

pub async fn find_user(
    pool: web::Data<DbPool>,
    id: i64
) -> Result<Option<UserEto>, Error> {
    let user = web::block(move || {
        let conn = pool.get()?;
        user_repository::find_by_id(id, &conn)
    })
        .await?;

    if let Some(user) = user {
        Ok(Some(UserEto::from(user)))
    } else {
        Ok(None)
    }
}

pub async fn find_users(
    pool: web::Data<DbPool>,
    criteria: UserSearchCriteria
) -> Result<SearchResult<UserEto>, Error> {
    let filters = criteria.clone();
    let pageable = filters.pageable.clone();

    let content = web::block(move || {
        let conn = pool.get()?;
        user_repository::find_by_criteria(filters, &conn)
    }).await?;

    let total_elements = content.len() as i32;
    let result_content = content.iter()
        .map(|user| UserEto::from(user.to_owned()))
        .collect();

    Ok(SearchResult::new(result_content, pageable, total_elements))
}

