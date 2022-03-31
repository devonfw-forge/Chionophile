use actix_web::{Error, web};
use async_trait::async_trait;
use crate::AppState;
use crate::core::general::search::search_result::SearchResult;
use crate::api::accesscodemanagement::logic::accesscode_management::AccessCodeManagement;
use crate::api::accesscodemanagement::logic::usecase::uc_find_accesscode::UcFindAccessCode;
use crate::api::accesscodemanagement::logic::usecase::uc_manage_accesscode::UcManageAccessCode;
use crate::core::accesscodemanagement::logic::api::accesscode_cto::AccessCodeCto;
use crate::core::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::core::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::core::accesscodemanagement::logic::usecase::uc_find_accesscode_impl::UcFindAccessCodeImpl;
use crate::core::accesscodemanagement::logic::usecase::uc_manage_accesscode_impl::UcManageAccessCodeImpl;
use crate::core::accesscodemanagement::rest::api::accesscode_post_data::AccessCodePostData;

pub struct AccessCodeManagementImpl;

impl AccessCodeManagement for AccessCodeManagementImpl {}

#[async_trait]
impl UcManageAccessCode for AccessCodeManagementImpl {
    async fn save_accesscode(
        app_state: web::Data<AppState>,
        accesscode_post_data: AccessCodePostData
    ) -> Result<AccessCodeEto, Error> {
        UcManageAccessCodeImpl::save_accesscode(app_state, accesscode_post_data).await
    }

    async fn delete_accesscode(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<(), Error> {
        UcManageAccessCodeImpl::delete_accesscode(app_state, id).await
    }
}

#[async_trait]
impl UcFindAccessCode for AccessCodeManagementImpl {
    async fn find_accesscode_cto(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<Option<AccessCodeCto>, Error> {
        UcFindAccessCodeImpl::find_accesscode_cto(app_state, id).await
    }

    async fn find_accesscode_ctos(
        app_state: web::Data<AppState>,
        criteria: AccessCodeSearchCriteria
    ) -> Result<SearchResult<AccessCodeCto>, Error> {
        UcFindAccessCodeImpl::find_accesscode_ctos(app_state, criteria).await
    }

    async fn find_accesscode_etos(
        app_state: web::Data<AppState>,
        criteria: AccessCodeSearchCriteria
    ) -> Result<SearchResult<AccessCodeEto>, Error> {
        UcFindAccessCodeImpl::find_accesscode_etos(app_state, criteria).await
    }

    async fn find_accesscode_eto(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<Option<AccessCodeEto>, Error> {
        UcFindAccessCodeImpl::find_accesscode_eto(app_state, id).await
    }
}
