use actix_web::{Error, web};
use actix_web::web::Data;
use async_trait::async_trait;
use crate::api::accesscodemanagement::dataaccess::accesscode_repository::AccessCodeRepository;
use crate::api::accesscodemanagement::logic::usecase::uc_find_accesscode::UcFindAccessCode;
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::AppState;
use crate::core::accesscodemanagement::dataaccess::api::repo::accesscode_repository_impl::AccessCodeRepositoryImpl;
use crate::core::accesscodemanagement::logic::api::accesscode_cto::AccessCodeCto;
use crate::core::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::core::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::core::general::search::search_result::SearchResult;

pub struct UcFindAccessCodeImpl;

#[async_trait]
impl UcFindAccessCode for UcFindAccessCodeImpl {
    async fn find_accesscode_cto(
        app_state: Data<AppState>,
        id: i64
    ) -> Result<Option<AccessCodeCto>, Error> {
        let access_code = web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::find_by_id_cto(id, &conn)
        })
            .await?;

        Ok(access_code)
    }

    async fn find_accesscode_ctos(
        app_state: Data<AppState>,
        criteria: AccessCodeSearchCriteria
    ) -> Result<SearchResult<AccessCodeCto>, Error> {
        let pageable = criteria.pageable.clone();

        let content = web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::find_ctos(criteria, &conn)
        }).await?;

        let total_elements = content.len() as i32;
        let search_results = SearchResult::new(content, pageable, total_elements);

        Ok(search_results)
    }

    async fn find_accesscode_etos(
        app_state: Data<AppState>,
        criteria: AccessCodeSearchCriteria
    ) -> Result<SearchResult<AccessCodeEto>, Error> {
        let pageable = criteria.pageable.clone();

        let content = web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::find_by_criteria(criteria, &conn)
        }).await?;

        let total_elements = content.len() as i32;
        let result_content = content.iter()
            .map(|accesscode| accesscode.clone().into())
            .collect();
        let search_results = SearchResult::new(result_content, pageable, total_elements);

        Ok(search_results)
    }

    async fn find_accesscode_eto(
        app_state: Data<AppState>,
        id: i64
    ) -> Result<Option<AccessCodeEto>, Error> {
        let accesscode = web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::find_by_id(id, &conn)
        })
            .await?;

        if let Some(accesscode) = accesscode {
            Ok(Some(accesscode.into()))
        } else {
            Ok(None)
        }
    }
}

