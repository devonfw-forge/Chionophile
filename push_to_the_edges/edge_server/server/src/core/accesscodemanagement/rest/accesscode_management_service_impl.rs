use actix_web::{HttpResponse, web, Error};
use actix_web::web::{Data, Json, Path};
use async_trait::async_trait;
use crate::api::accesscodemanagement::logic::usecase::uc_find_accesscode::UcFindAccessCode;
use crate::api::accesscodemanagement::logic::usecase::uc_manage_accesscode::UcManageAccessCode;
use crate::api::accesscodemanagement::rest::api::accesscode_management_service::AccessCodeManagementService;
use crate::api::common::errors::save_error::SaveError;
use crate::api::common::rest::crud_rest_service::CRUDRestService;
use crate::AppState;
use crate::core::accesscodemanagement::logic::accesscode_management_impl::AccessCodeManagementImpl;
use crate::core::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::core::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::core::accesscodemanagement::rest::api::accesscode_post_data::AccessCodePostData;

pub struct AccessCodeManagementServiceImpl;

#[async_trait]
impl AccessCodeManagementService for AccessCodeManagementServiceImpl {
    async fn find_accesscode_cto(
        app_state: Data<AppState>,
        id: Path<i64>
    ) -> Result<HttpResponse, Error> {
        let id = id.into_inner();
        let accesscode_id = id.clone();
        let access_code = AccessCodeManagementImpl::find_accesscode_cto(app_state, id)
            .await?;

        if let Some(access_code) = access_code {
            Ok(HttpResponse::Ok().json(access_code))
        } else {
            let res = HttpResponse::NotFound()
                .body(format!("No access_code found with id: {}", accesscode_id));
            Ok(res)
        }
    }

    async fn find_accesscode_ctos(
        app_state: Data<AppState>,
        criteria: Json<AccessCodeSearchCriteria>
    ) -> Result<HttpResponse, Error> {
        let search_results = AccessCodeManagementImpl::
        find_accesscode_ctos(app_state, criteria.into_inner())
            .await?;

        Ok(HttpResponse::Ok().json(search_results))
    }
}

#[async_trait]
impl CRUDRestService<i64, AccessCodeEto, AccessCodeSearchCriteria, AccessCodePostData> for AccessCodeManagementServiceImpl {
    async fn search(
        app_state: Data<AppState>,
        criteria: web::Json<AccessCodeSearchCriteria>
    ) -> Result<HttpResponse, Error> {
        let search_results = AccessCodeManagementImpl::
        find_accesscode_etos(app_state, criteria.into_inner())
            .await?;

        Ok(HttpResponse::Ok().json(search_results))
    }

    async fn get(
        app_state: Data<AppState>,
        id: Path<i64>
    ) -> Result<HttpResponse, Error> {
        let accesscode = AccessCodeManagementImpl::
        find_accesscode_eto(app_state, id.into_inner())
            .await?;

        if let Some(accesscode) = accesscode {
            Ok(HttpResponse::Ok().json(accesscode))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }

    async fn save(
        app_state: Data<AppState>,
        accesscode: web::Json<AccessCodePostData>
    ) -> Result<HttpResponse, Error> {
        let save_result: Result<AccessCodeEto, SaveError> = AccessCodeManagementImpl::
        save_accesscode(app_state, accesscode.into_inner())
            .await;

        match save_result {
            Ok(accesscode) => {
                Ok(HttpResponse::Ok().json(accesscode))
            }
            Err(save_error) => {
                match save_error {
                    SaveError::ValidationErrors(validation_errors) => {
                        Ok(HttpResponse::BadRequest().json(validation_errors))
                    }
                    SaveError::InternalServerError => {
                        Ok(HttpResponse::InternalServerError().finish())
                    }
                }
            }
        }
    }

    async fn delete(
        app_state: Data<AppState>,
        id: Path<i64>
    ) -> Result<HttpResponse, Error> {
        let deleted_id = AccessCodeManagementImpl::delete_accesscode(app_state, id.into_inner())
            .await?;

        if let Some(id) = deleted_id {
            Ok(HttpResponse::Ok().body(id.to_string()))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }
}
