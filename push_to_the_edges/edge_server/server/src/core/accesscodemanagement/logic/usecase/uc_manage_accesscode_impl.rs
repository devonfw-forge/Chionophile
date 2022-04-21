use actix_web::{Error, web};
use actix_web::web::{Data};
use async_trait::async_trait;
use crate::api::accesscodemanagement::logic::usecase::uc_manage_accesscode::UcManageAccessCode;
use crate::api::common::dataaccess::api::repo::cache_repository::CacheRepository;
use crate::api::common::errors::save_error::SaveError;
use crate::api::queuemanagement::logic::usecase::uc_manage_queue::UcManageQueue;
use crate::api::visitormanagement::logic::usecase::uc_manage_visitor::UcManageVisitor;
use crate::AppState;
use crate::core::accesscodemanagement::dataaccess::api::accesscode::AccessCode;
use crate::core::accesscodemanagement::dataaccess::api::repo::accesscode_repository_impl::AccessCodeRepositoryImpl;
use crate::core::accesscodemanagement::logic::api::accesscode_cto::AccessCodeCto;
use crate::core::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::core::accesscodemanagement::rest::api::accesscode_post_data::AccessCodePostData;
use crate::core::queuemanagement::logic::usecase::uc_manage_queue_impl::UcManageQueueImpl;
use crate::core::visitormanagement::logic::usecase::uc_manage_visitor_impl::UcManageVisitorImpl;

pub struct UcManageAccessCodeImpl;

#[async_trait]
impl UcManageAccessCode for UcManageAccessCodeImpl {
    async fn save_accesscode(
        app_state: Data<AppState>,
        accesscode_post_data: AccessCodePostData
    ) -> Result<AccessCodeEto, SaveError> {
        let request_url =
            format!("{}/jumpthequeue/services/rest/accesscodemanagement/v1/accesscode/",
                    app_state.central_url);
        let client = reqwest::Client::new();

        let result = client.post(request_url).json(&accesscode_post_data).send().await?;

        if result.status() != 200 {
            return Err(SaveError::InternalServerError)
        }

        let accesscode: AccessCodeEto = result.json().await?;

        Self::cache_accesscode(app_state, accesscode.clone()).await?;

        Ok(accesscode)
    }

    async fn cache_accesscode(
        app_state: Data<AppState>,
        accesscode: AccessCodeEto
    )-> Result<AccessCodeEto, SaveError> {
        let access_code = web::block(move || {
            let conn = app_state.pool.get()?;
            let accesscode_entity = AccessCode::from(accesscode);
            AccessCodeRepositoryImpl::save(&accesscode_entity, &conn)
        }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(access_code.into())
    }

    async fn cache_accesscode_cto(
        app_state: Data<AppState>,
        accesscode: AccessCodeCto
    ) -> Result<AccessCodeCto, SaveError> {
        let res = accesscode.clone();
        UcManageQueueImpl::cache_queue(app_state.clone(), accesscode.queue).await?;
        UcManageVisitorImpl::cache_visitor(app_state.clone(), accesscode.visitor).await?;
        Self::cache_accesscode(app_state.clone(), accesscode.access_code).await?;

        Ok(res)
    }

    async fn delete_accesscode(
        app_state: Data<AppState>,
        id: i64
    ) -> Result<Option<i64>, Error> {
        let central_url = app_state.central_url.clone();
        let deleted = web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::delete_by_id(id, &conn)
        }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

        let request_url =
            format!("{}/jumpthequeue/services/rest/accesscodemanagement/v1/accesscode/{}/",
                    central_url, id.to_string());

        let client = reqwest::Client::new();

        client.delete(request_url).send().await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(deleted)
    }
}

