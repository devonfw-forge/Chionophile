use actix_web::{HttpResponse, web, Error};
use actix_web::web::{Data, Path};
use crate::core::visitormanagement::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use crate::core::visitormanagement::logic::api::visitor_eto::VisitorEto;
use async_trait::async_trait;
use crate::api::common::rest::crud_rest_service::CRUDRestService;
use crate::api::visitormanagement::logic::api::errors::save_error::SaveError;
use crate::api::visitormanagement::logic::usecase::uc_find_visitor::UcFindVisitor;
use crate::api::visitormanagement::logic::usecase::uc_manage_visitor::UcManageVisitor;
use crate::api::visitormanagement::rest::api::visitor_management_service::VisitorManagementService;
use crate::AppState;
use crate::core::visitormanagement::logic::visitor_management_impl::VisitorManagementImpl;

pub struct VisitorManagementServiceImpl;

impl VisitorManagementService for VisitorManagementServiceImpl {}

#[async_trait]
impl CRUDRestService<i64, VisitorEto, VisitorSearchCriteria, VisitorEto> for VisitorManagementServiceImpl {
    async fn search(
        app_state: Data<AppState>,
        criteria: web::Json<VisitorSearchCriteria>
    ) -> Result<HttpResponse, Error> {
        let search_results =
            VisitorManagementImpl
            ::find_visitors(app_state, criteria.into_inner())
                .await?;

        Ok(HttpResponse::Ok().json(search_results))
    }

    async fn get(
        app_state: Data<AppState>,
        id: Path<i64>
    ) -> Result<HttpResponse, Error> {
        let id = id.into_inner();
        let visitor_id = id.clone();
        let visitor =
            VisitorManagementImpl
            ::find_visitor(app_state, visitor_id)
                .await?;

        if let Some(visitor) = visitor {
            Ok(HttpResponse::Ok().json(visitor))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }

    async fn save(
        app_state: Data<AppState>,
        visitor: web::Json<VisitorEto>
    ) -> Result<HttpResponse, Error> {
        println!("SAVE USER");
        let save_result: Result<VisitorEto, SaveError> =
            VisitorManagementImpl
            ::save_visitor(app_state, visitor.into_inner())
                .await;

        match save_result {
            Ok(visitor) => {
                Ok(HttpResponse::Ok().json(visitor))
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
        let deleted_id = VisitorManagementImpl::
        delete_visitor(app_state, id.into_inner())
            .await?;

        if let Some(id) = deleted_id {
            Ok(HttpResponse::Ok().body(id.to_string()))
        } else {
            Ok(HttpResponse::NotFound().finish())
        }
    }
}
