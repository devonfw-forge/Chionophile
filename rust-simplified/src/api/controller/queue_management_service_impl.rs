use crate::{
    api::{controller::crud_rest_service::CRUDRestService, model::queue_eto::QueueEto},
    domain::{
        models::{new_queue::NewQueue, queue::Queue},
        repositories::{queue_repository_impl::QueueRepositoryImpl, repository::Repository},
        search::search_result::SearchResult,
        tos::queue_search_criteria::QueueSearchCriteria,
    },
    AppState,
};
use actix_web::web::{Data, Path};
use actix_web::{web, Error, HttpResponse};
use async_trait::async_trait;

pub struct QueueManagementServiceImpl;

#[async_trait]
impl CRUDRestService<i64, QueueEto, QueueSearchCriteria, QueueEto> for QueueManagementServiceImpl {
    async fn search(
        app_state: Data<AppState>,
        criteria: web::Json<QueueSearchCriteria>,
    ) -> Result<HttpResponse, Error> {
        let filters = criteria.clone();
        let pageable = filters.pageable.clone();

        let query_results: Vec<Queue> = web::block(move || {
            let conn = app_state.pool.get()?;
            QueueRepositoryImpl::find_by_criteria(filters, &conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        let total_elements = query_results.len() as i32;
        let paged_results = criteria.pageable.from(query_results);
        let result_content: Vec<QueueEto> = paged_results
            .iter()
            .map(|queue| queue.clone().into())
            .collect();

        let search_result = SearchResult::new(result_content, pageable, total_elements);

        Ok(HttpResponse::Ok().json(search_result))
    }

    async fn get(app_state: Data<AppState>, id: Path<i64>) -> Result<HttpResponse, Error> {
        let id = id.into_inner();
        let queue: Option<Queue> = web::block(move || {
            let conn = app_state.pool.get()?;
            QueueRepositoryImpl::find_by_id(id, &conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        if let Some(queue) = queue {
            Ok(HttpResponse::Ok().json(<Queue as Into<QueueEto>>::into(queue)))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }

    async fn save(
        app_state: Data<AppState>,
        queue: web::Json<QueueEto>,
    ) -> Result<HttpResponse, Error> {
        let result = web::block(move || {
            let conn = app_state.pool.get()?;
            let new_queue: NewQueue = NewQueue::from(queue.into_inner());
            QueueRepositoryImpl::save(&new_queue, &conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(<Queue as Into<QueueEto>>::into(result)))
    }

    async fn delete(app_state: Data<AppState>, id: Path<i64>) -> Result<HttpResponse, Error> {
        let deleted_id = web::block(move || {
            let conn = app_state.pool.get()?;
            QueueRepositoryImpl::delete_by_id(id.into_inner(), &conn)
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
