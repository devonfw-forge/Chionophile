use actix_web::{Error, web};
use async_trait::async_trait;
use crate::api::common::dataaccess::api::repo::cache_repository::CacheRepository;
use crate::api::queuemanagement::logic::usecase::uc_find_queue::UcFindQueue;
use crate::api::queuemanagement::logic::usecase::uc_manage_queue::UcManageQueue;
use crate::AppState;
use crate::core::general::search::search_result::SearchResult;
use crate::core::queuemanagement::dataaccess::api::queue::Queue;
use crate::core::queuemanagement::dataaccess::api::repo::queue_repository_impl::QueueRepositoryImpl;
use crate::core::queuemanagement::logic::api::queue_search_criteria::QueueSearchCriteria;
use crate::core::queuemanagement::logic::api::queue_eto::QueueEto;
use crate::core::queuemanagement::logic::usecase::uc_manage_queue_impl::UcManageQueueImpl;

pub struct UcFindQueueImpl;

#[async_trait]
impl UcFindQueue for UcFindQueueImpl {
    async fn find_queue(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<Option<QueueEto>, Error> {
        let request_state = app_state.clone();

        let queue = web::block(move || {
            let conn = app_state.pool.get()?;
            QueueRepositoryImpl::find_by_id(id, &conn)
        }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

        if let Some(queue) = queue {
            return Ok(Some(queue.into()));
        }

        println!("No queue in edge");
        let request_url =
            format!("{}/jumpthequeue/services/rest/queuemanagement/v1/queue/{}/",
                    &request_state.central_url, id);

        let response = reqwest::get(&request_url).await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        if response.status() == 404 {
            println!("No queue in central");
            return Ok(None)
        } else {
            let queue: QueueEto = response.json().await
                .map_err(actix_web::error::ErrorInternalServerError)?;

            UcManageQueueImpl::cache_queue(request_state, queue.clone()).await
                .map_err(actix_web::error::ErrorInternalServerError)?;

            Ok(Some(queue))
        }


    }

    async fn find_queues(
        app_state: web::Data<AppState>,
        criteria: QueueSearchCriteria
    ) -> Result<SearchResult<QueueEto>, Error> {
        let request_url =
            format!("{}/jumpthequeue/services/rest/queuemanagement/v1/queue/search/",
                    app_state.central_url);

        let client = reqwest::Client::new();
        let response = client.post(request_url).json(&criteria).send().await
            .map_err(actix_web::error::ErrorInternalServerError)?;
        let search_results: SearchResult<QueueEto> = response.json().await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        let cached_queues: Vec<Queue> = search_results.content
            .iter()
            .map(|queue_eto| Queue::from(queue_eto.clone()))
            .collect::<Vec<Queue>>();

        let _inserted_queues = web::block(move || {
            let conn = app_state.pool.get()?;
            QueueRepositoryImpl::insert_all(cached_queues, &conn)
        }).await.map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(search_results)
    }
}

