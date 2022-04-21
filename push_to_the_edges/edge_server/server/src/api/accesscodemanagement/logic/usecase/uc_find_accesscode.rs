use actix_web::{Error, web};
use async_trait::async_trait;
use crate::AppState;
use crate::core::accesscodemanagement::logic::api::accesscode_cto::AccessCodeCto;
use crate::core::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::core::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::core::general::search::search_result::SearchResult;

#[async_trait]
pub trait UcFindAccessCode {
    async fn find_accesscode_cto(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<Option<AccessCodeCto>, Error>;

    async fn find_accesscode_ctos(
        app_state: web::Data<AppState>,
        criteria: AccessCodeSearchCriteria
    ) -> Result<SearchResult<AccessCodeCto>, Error>;

    async fn find_accesscode_etos(
        app_state: web::Data<AppState>,
        criteria: AccessCodeSearchCriteria
    ) -> Result<SearchResult<AccessCodeEto>, Error>;

    async fn find_accesscode_eto(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<Option<AccessCodeEto>, Error>;
    
}