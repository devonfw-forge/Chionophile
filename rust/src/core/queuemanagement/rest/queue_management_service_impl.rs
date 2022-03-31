use actix_web::{HttpResponse, web, Error};
use actix_web::web::{Data, Path};
use crate::core::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;
use crate::core::queuemanagement::logic::api::queue_eto::QueueEto;
use async_trait::async_trait;
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
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

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
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

        if let Some(queue) = queue {
            Ok(HttpResponse::Ok().json(queue))
        } else {
            let res = HttpResponse::NotFound()
                .body(format!("No queue found with uid: {}", id));
            Ok(res)
        }
    }

    async fn save(
        app_state: Data<AppState>,
        queue: web::Json<QueueEto>
    ) -> Result<HttpResponse, Error> {
            let queue = QueueManagementImpl::
            save_queue(app_state, queue.into_inner())
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(queue))

    }

    async fn delete(
        app_state: Data<AppState>,
        id: Path<i64>
    ) -> Result<HttpResponse, Error> {
        QueueManagementImpl::
        delete_queue(app_state, id.into_inner())
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().finish())
    }
}
