use actix_web::{Error, HttpResponse, web};
use crate::api::common::rest::crud_rest_service::CRUDRestService;
use crate::AppState;
use async_trait::async_trait;
use crate::core::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::core::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::core::accesscodemanagement::rest::api::accesscode_post_data::AccessCodePostData;

#[async_trait]
pub trait AccessCodeManagementService:  CRUDRestService<i64, AccessCodeEto, AccessCodeSearchCriteria, AccessCodePostData> {

    async fn find_accesscode_cto(
        app_state: web::Data<AppState>,
        id: web::Path<i64>
    ) -> Result<HttpResponse, Error>;

    async fn find_accesscode_ctos(
        app_state: web::Data<AppState>,
        criteria: web::Json<AccessCodeSearchCriteria>
    ) -> Result<HttpResponse, Error>;

}
