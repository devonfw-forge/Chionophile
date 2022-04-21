use actix_web::{Error, web};
use async_trait::async_trait;
use crate::AppState;
use crate::core::general::search::search_result::SearchResult;
use crate::core::visitormanagement::logic::api::visitor_eto::VisitorEto;
use crate::core::visitormanagement::logic::api::visitor_search_criteria::VisitorSearchCriteria;

#[async_trait]
pub trait UcFindVisitor {
    async fn find_visitor(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<Option<VisitorEto>, Error>;

    async fn find_visitors(
        app_state: web::Data<AppState>,
        criteria: VisitorSearchCriteria
    ) -> Result<SearchResult<VisitorEto>, Error>;
}