use crate::{
    api::controller::crud_rest_service::CRUDRestService,
    domain::{
        models::new_accesscode::NewAccessCode, repositories::repository::Repository,
        search::search_result::SearchResult, state::app_state::AppState,
        tos::accesscode_search_criteria::AccessCodeSearchCriteria,
    },
};
use crate::{
    api::model::{accesscode_eto::AccessCodeEto, saveable::Saveable},
    domain::repositories::accesscode_repository_impl::AccessCodeRepositoryImpl,
};
use actix_web::web::{Data, Json, Path};
use actix_web::{web, Error, HttpResponse};
use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Serialize, Saveable)]
#[serde(rename_all = "camelCase")]
pub struct AccessCodePostData {
    pub visitor_id: i64,
    pub queue_id: i64,
}

pub struct AccessCodeManagementServiceImpl;

impl AccessCodeManagementServiceImpl {
    pub async fn find_accesscode_cto(
        app_state: Data<AppState>,
        id: Path<i64>,
    ) -> Result<HttpResponse, Error> {
        let id = id.into_inner();
        let access_code = web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::find_by_id_cto(id, &conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        if let Some(access_code) = access_code {
            Ok(HttpResponse::Ok().json(access_code))
        } else {
            let res =
                HttpResponse::NotFound().body(format!("No access_code found with id: {}", id));
            Ok(res)
        }
    }

    pub async fn find_accesscode_ctos(
        app_state: Data<AppState>,
        criteria: Json<AccessCodeSearchCriteria>,
    ) -> Result<HttpResponse, Error> {
        let criteria = criteria.into_inner();

        let pageable = criteria.pageable.clone();

        let content = web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::find_ctos(criteria, &conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        let total_elements = content.len() as i32;
        let paged_results = pageable.from(content);
        let search_result = SearchResult::new(paged_results, pageable, total_elements);

        Ok(HttpResponse::Ok().json(search_result))
    }
}

#[async_trait]
impl CRUDRestService<i64, AccessCodeEto, AccessCodeSearchCriteria, AccessCodePostData>
    for AccessCodeManagementServiceImpl
{
    async fn search(
        app_state: Data<AppState>,
        criteria: web::Json<AccessCodeSearchCriteria>,
    ) -> Result<HttpResponse, Error> {
        let criteria = criteria.into_inner();
        let pageable = criteria.pageable.clone();

        let content = web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::find_by_criteria(criteria, &conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        let total_elements = content.len() as i32;
        let paged_results = pageable.from(content);
        let result_content: Vec<AccessCodeEto> = paged_results
            .iter()
            .map(|accesscode| accesscode.clone().into())
            .collect();
        let search_result = SearchResult::new(result_content, pageable, total_elements);

        Ok(HttpResponse::Ok().json(search_result))
    }

    async fn get(app_state: Data<AppState>, id: Path<i64>) -> Result<HttpResponse, Error> {
        let accesscode = web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::find_by_id(id.into_inner(), &conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        if let Some(accesscode) = accesscode {
            Ok(HttpResponse::Ok().json(accesscode))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }

    async fn save(
        app_state: Data<AppState>,
        accesscode: web::Json<AccessCodePostData>,
    ) -> Result<HttpResponse, Error> {
        let access_code = web::block(move || {
            let conn = app_state.pool.get()?;

            let mut access_code_entity = NewAccessCode {
                modification_counter: 1,
                creation_time: None,
                start_time: None,
                end_time: None,
                queue_id: accesscode.queue_id.clone(),
                visitor_id: accesscode.visitor_id.clone(),
            };

            access_code_entity.creation_time = Some(Utc::now().naive_utc());
            AccessCodeRepositoryImpl::save(&access_code_entity, &conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        let eto: AccessCodeEto = access_code.into();

        Ok(HttpResponse::Ok().json(eto))
    }

    async fn delete(app_state: Data<AppState>, id: Path<i64>) -> Result<HttpResponse, Error> {
        let deleted_id = web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::delete_by_id(id.into_inner(), &conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        if let Some(id) = deleted_id {
            Ok(HttpResponse::Ok().body(id.to_string()))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }
}
