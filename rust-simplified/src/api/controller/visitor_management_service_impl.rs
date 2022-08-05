use crate::api::{controller::crud_rest_service::CRUDRestService, errors::save_error::SaveError};
use crate::domain::models::visitor::Visitor;
use crate::domain::repositories::repository::Repository;
use crate::domain::tos::visitor_search_criteria::VisitorSearchCriteria;
use crate::domain::{
    models::new_visitor::NewVisitor, repositories::visitor_repository_impl::VisitorRepositoryImpl,
};
use crate::AppState;
use crate::{api::model::visitor_eto::VisitorEto, domain::search::search_result::SearchResult};
use actix_web::web::{Data, Path};
use actix_web::{web, Error, HttpResponse};
use async_trait::async_trait;
use validator::Validate;

pub struct VisitorManagementServiceImpl;

#[async_trait]
impl CRUDRestService<i64, VisitorEto, VisitorSearchCriteria, VisitorEto>
    for VisitorManagementServiceImpl
{
    async fn search(
        app_state: Data<AppState>,
        criteria: web::Json<VisitorSearchCriteria>,
    ) -> Result<HttpResponse, Error> {
        let filters = criteria.clone();
        let pageable = filters.pageable.clone();

        let query_results: Vec<Visitor> = web::block(move || {
            let conn = app_state.pool.get()?;
            VisitorRepositoryImpl::find_by_criteria(filters, &conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        let total_elements = query_results.len() as i32;
        let paged_results = criteria.pageable.from(query_results);
        let result_content: Vec<VisitorEto> = paged_results
            .iter()
            .map(|visitor| visitor.clone().into())
            .collect();

        let search_result = SearchResult::new(result_content, pageable, total_elements);

        Ok(HttpResponse::Ok().json(search_result))
    }

    async fn get(app_state: Data<AppState>, id: Path<i64>) -> Result<HttpResponse, Error> {
        let id = id.into_inner();
        let visitor: Option<Visitor> = web::block(move || {
            let conn = app_state.pool.get()?;
            VisitorRepositoryImpl::find_by_id(id, &conn)
        })
        .await?
        .map_err(actix_web::error::ErrorInternalServerError)?;

        if let Some(visitor) = visitor {
            Ok(HttpResponse::Ok().json(<Visitor as Into<VisitorEto>>::into(visitor)))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }

    async fn save(
        app_state: Data<AppState>,
        visitor: web::Json<VisitorEto>,
    ) -> Result<HttpResponse, Error> {
        let result: Result<Visitor, SaveError> = web::block(move || {
            let conn = app_state.pool.get()?;
            visitor.validate()?;
            let new_visitor: NewVisitor = NewVisitor::from(visitor.into_inner());
            let res = VisitorRepositoryImpl::save(&new_visitor, &conn)?;

            Ok(res)
        })
        .await?;

        match result {
            Ok(visitor) => {
                Ok(HttpResponse::Ok().json(<Visitor as Into<VisitorEto>>::into(visitor)))
            }
            Err(save_error) => match save_error {
                SaveError::ValidationErrors(validation_errors) => {
                    Ok(HttpResponse::BadRequest().json(validation_errors))
                }
                SaveError::InternalServerError => Ok(HttpResponse::InternalServerError().finish()),
            },
        }
    }

    async fn delete(app_state: Data<AppState>, id: Path<i64>) -> Result<HttpResponse, Error> {
        let deleted_id = web::block(move || {
            let conn = app_state.pool.get()?;
            VisitorRepositoryImpl::delete_by_id(id.into_inner(), &conn)
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
