use actix_web::{Error, web};
use async_trait::async_trait;
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::api::usermanagement::logic::usecase::uc_find_user::UcFindUser;
use crate::AppState;
use crate::core::general::search::search_result::SearchResult;
use crate::core::general::security::hashser::hash_password;
use crate::core::usermanagement::dataaccess::api::repo::user_repository_impl::UserRepositoryImpl;
use crate::core::usermanagement::logic::api::user_search_criteria::UserSearchCriteria;
use crate::core::usermanagement::logic::api::user_eto::UserEto;

pub struct UcFindUserImpl;

#[async_trait]
impl UcFindUser for UcFindUserImpl {
    async fn find_user(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<Option<UserEto>, Error> {
        let user = web::block(move || {
            let conn = app_state.pool.get()?;
            UserRepositoryImpl::find_by_id(id, &conn)
        })
            .await?;

        if let Some(user) = user {
            Ok(Some(user.into()))
        } else {
            Ok(None)
        }
    }

    async fn find_users(
        app_state: web::Data<AppState>,
        mut criteria: UserSearchCriteria
    ) -> Result<SearchResult<UserEto>, Error> {
        let filters = criteria.clone();
        let pageable = filters.pageable.clone();

        let query_results = web::block(move || {
            let conn = app_state.pool.get()?;
            if let Some(password) = criteria.password {
                let hashed_password = hash_password(&password);
                criteria.password = Some(hashed_password);
            }
            UserRepositoryImpl::find_by_criteria(filters, &conn)
        }).await?;

        let total_elements = query_results.len() as i32;
        let paged_results = criteria.pageable.from(query_results);
        let result_content = paged_results.iter()
            .map(|user| user.clone().into())
            .collect();

        Ok(SearchResult::new(result_content, pageable, total_elements))
    }
}


