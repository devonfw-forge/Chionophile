use actix_web::{Error, web};
use async_trait::async_trait;
use crate::api::visitormanagement::logic::api::errors::save_error::SaveError;
use crate::api::visitormanagement::logic::usecase::uc_find_visitor::UcFindVisitor;
use crate::api::visitormanagement::logic::usecase::uc_manage_visitor::UcManageVisitor;
use crate::api::visitormanagement::logic::visitor_management::VisitorManagement;
use crate::AppState;
use crate::core::general::search::search_result::SearchResult;
use crate::core::visitormanagement::logic::api::visitor_search_criteria::VisitorSearchCriteria;
use crate::core::visitormanagement::logic::api::visitor_eto::VisitorEto;
use crate::core::visitormanagement::logic::usecase::uc_find_visitor_impl::UcFindVisitorImpl;
use crate::core::visitormanagement::logic::usecase::uc_manage_visitor_impl::UcManageVisitorImpl;

pub struct VisitorManagementImpl;

impl VisitorManagement for VisitorManagementImpl {}

#[async_trait]
impl UcManageVisitor for VisitorManagementImpl {
    async fn save_visitor(
        app_state: web::Data<AppState>,
        visitor: VisitorEto
    ) -> Result<VisitorEto, SaveError> {
        UcManageVisitorImpl::save_visitor(app_state, visitor).await
    }

    async fn delete_visitor(
        app_state: web::Data<AppState>,
        visitor_id: i64
    ) -> Result<(), Error> {
        UcManageVisitorImpl::delete_visitor(app_state, visitor_id).await
    }

}

#[async_trait]
impl UcFindVisitor for VisitorManagementImpl {
    async fn find_visitor(
        app_state: web::Data<AppState>,
        id: i64,
    ) -> Result<Option<VisitorEto>, Error> {
        UcFindVisitorImpl::find_visitor(app_state, id).await
    }

    async fn find_visitors(
        app_state: web::Data<AppState>,
        criteria: VisitorSearchCriteria
    ) -> Result<SearchResult<VisitorEto>, Error> {
        UcFindVisitorImpl::find_visitors(app_state, criteria).await
    }
}

