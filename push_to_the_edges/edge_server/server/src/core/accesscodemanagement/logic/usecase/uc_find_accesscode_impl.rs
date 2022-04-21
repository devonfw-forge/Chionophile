use actix_web::{Error, web};
use actix_web::web::Data;
use async_trait::async_trait;
use futures::executor::block_on;
use crate::api::accesscodemanagement::logic::usecase::uc_find_accesscode::UcFindAccessCode;
use crate::api::accesscodemanagement::logic::usecase::uc_manage_accesscode::UcManageAccessCode;
use crate::api::common::dataaccess::api::repo::cache_repository::CacheRepository;
use crate::api::common::errors::save_error::SaveError;
use crate::api::queuemanagement::logic::usecase::uc_find_queue::UcFindQueue;
use crate::api::visitormanagement::logic::usecase::uc_find_visitor::UcFindVisitor;
use crate::AppState;
use crate::core::accesscodemanagement::dataaccess::api::accesscode::AccessCode;
use crate::core::accesscodemanagement::dataaccess::api::repo::accesscode_repository_impl::AccessCodeRepositoryImpl;
use crate::core::accesscodemanagement::logic::api::accesscode_cto::AccessCodeCto;
use crate::core::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::core::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::core::accesscodemanagement::logic::usecase::uc_manage_accesscode_impl::UcManageAccessCodeImpl;
use crate::core::general::search::search_result::SearchResult;
use crate::core::queuemanagement::dataaccess::api::queue::Queue;
use crate::core::queuemanagement::logic::usecase::uc_find_queue_impl::UcFindQueueImpl;
use crate::core::visitormanagement::dataaccess::api::visitor::Visitor;
use crate::core::visitormanagement::logic::usecase::uc_find_visitor_impl::UcFindVisitorImpl;

pub struct UcFindAccessCodeImpl;

#[async_trait]
impl UcFindAccessCode for UcFindAccessCodeImpl {
    async fn find_accesscode_cto(
        app_state: Data<AppState>,
        id: i64
    ) -> Result<Option<AccessCodeCto>, Error> {
        let request_state = app_state.clone();

        let accesscode = web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::find_by_id(id, &conn)
        }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

        if let Some(accesscode) = accesscode {
            let visitor = UcFindVisitorImpl::find_visitor(request_state.clone(), accesscode.visitor_id.clone())
                .await.map_err(actix_web::error::ErrorInternalServerError)?;
            if let Some(visitor) = visitor {
                let queue = UcFindQueueImpl::find_queue(request_state.clone(), accesscode.queue_id.clone())
                    .await.map_err(actix_web::error::ErrorInternalServerError)?;
                if let Some(queue) = queue {
                    let result = AccessCodeCto::from_query_result((accesscode, Visitor::from(visitor), Queue::from(queue)));

                    return Ok(Option::from(result));
                }
            }
        }

        let request_url =
            format!("{}/jumpthequeue/services/rest/accesscodemanagement/v1/accesscode/cto/{}/",
                    request_state.central_url, id);
        let client = reqwest::Client::new();
        let response = client.get(request_url).send().await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let accesscode_cto: AccessCodeCto = response.json()
            .await.map_err(actix_web::error::ErrorInternalServerError)?;

        UcManageAccessCodeImpl::cache_accesscode_cto(request_state, accesscode_cto.clone())
            .await.map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(Option::from(accesscode_cto))
    }

    async fn find_accesscode_ctos(
        app_state: Data<AppState>,
        criteria: AccessCodeSearchCriteria
    ) -> Result<SearchResult<AccessCodeCto>, Error> {
        let request_url =
            format!("{}/jumpthequeue/services/rest/accesscodemanagement/v1/accesscode/cto/search/",
                    app_state.central_url);
        let client = reqwest::Client::new();
        let response = client.post(request_url).json(&criteria).send().await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        let search_results: SearchResult<AccessCodeCto> = response.json().await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        //Ignore the errors in caching if we can return a correct result
        let _cache_results: Vec<Result<AccessCodeCto, SaveError>> = search_results.content.iter().map(|cto| {
            block_on(UcManageAccessCodeImpl::cache_accesscode_cto(app_state.clone(), cto.clone()))
        }).collect();

        Ok(search_results)
    }

    async fn find_accesscode_etos(
        app_state: Data<AppState>,
        criteria: AccessCodeSearchCriteria
    ) -> Result<SearchResult<AccessCodeEto>, Error> {
        let request_url =
            format!("{}/jumpthequeue/services/rest/accesscodemanagement/v1/accesscode/search/",
                    app_state.central_url);

        let client = reqwest::Client::new();
        let response = client.post(request_url).json(&criteria).send().await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        let search_results: SearchResult<AccessCodeEto> = response.json().await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let cached_accesscodes: Vec<AccessCode> = search_results.content
            .iter()
            .map(|accesscode_eto| AccessCode::from(accesscode_eto.clone()))
            .collect::<Vec<AccessCode>>();

        let _inserted_accesscodes = web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::insert_all(cached_accesscodes, &conn)
        }).await.map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(search_results)
    }

    async fn find_accesscode_eto(
        app_state: Data<AppState>,
        id: i64
    ) -> Result<Option<AccessCodeEto>, Error> {
        let request_state = app_state.clone();

        let result = web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::find_by_id(id, &conn)
        }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

        if let Some(accesscode_res) = result {
            return Ok(Some(accesscode_res.into()))
        }
        println!("No accesscode in edge");
        let request_url =
            format!("{}/jumpthequeue/services/rest/accesscodemanagement/v1/accesscode/{}/",
                    &request_state.central_url, id);

        let response = reqwest::get(&request_url).await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        if response.status() == 404 {
            println!("No accesscode in central");
            return Ok(None)
        } else {
            let result: AccessCodeEto = response.json().await
                .map_err(actix_web::error::ErrorInternalServerError)?;

            UcManageAccessCodeImpl::cache_accesscode(request_state, result.clone()).await
                .map_err(actix_web::error::ErrorInternalServerError)?;

            Ok(Some(result))
        }
    }
}

