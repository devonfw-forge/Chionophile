use actix_web::{HttpResponse, web, Error};
use actix_web::web::{Data, Path};
use crate::core::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;
use crate::core::queuemanagement::logic::api::queue_eto::QueueEto;
use async_trait::async_trait;
use crate::api::common::errors::save_error::SaveError;
use crate::api::common::rest::crud_rest_service::CRUDRestService;
use crate::api::queuemanagement::logic::usecase::uc_find_queue::UcFindQueue;
use crate::api::queuemanagement::logic::usecase::uc_manage_queue::UcManageQueue;
use crate::api::queuemanagement::rest::api::queue_management_service::QueueManagementService;
use crate::AppState;
use crate::core::queuemanagement::logic::queue_management_impl::QueueManagementImpl;

pub struct QueueManagementServiceImpl;

impl QueueManagementService for QueueManagementServiceImpl {}

#[async_trait]
impl CRUDRestService<i64, QueueEto, QueueSearchCriteria, QueueEto> for QueueManagementServiceImpl {
    async fn search(
        app_state: Data<AppState>,
        criteria: web::Json<QueueSearchCriteria>
    ) -> Result<HttpResponse, Error> {
        let search_results =
            QueueManagementImpl
            ::find_queues(app_state, criteria.into_inner())
                .await?;

        Ok(HttpResponse::Ok().json(search_results))
    }

    async fn get(
        app_state: Data<AppState>,
        id: Path<i64>
    ) -> Result<HttpResponse, Error> {
        let id = id.into_inner();
        let queue_id = id.clone();
        let queue =
            QueueManagementImpl
            ::find_queue(app_state, queue_id)
                .await?;

        if let Some(queue) = queue {
            Ok(HttpResponse::Ok().json(queue))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }

    async fn save(
        app_state: Data<AppState>,
        queue: web::Json<QueueEto>
    ) -> Result<HttpResponse, Error> {
            let save_result: Result<QueueEto, SaveError> = QueueManagementImpl::
            save_queue(app_state, queue.into_inner())
                .await;

        match save_result {
            Ok(queue) => {
                Ok(HttpResponse::Ok().json(queue))
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
        let deleted_id = QueueManagementImpl::
        delete_queue(app_state, id.into_inner())
            .await?;

        if let Some(id) = deleted_id {
            Ok(HttpResponse::Ok().body(id.to_string()))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }
}
