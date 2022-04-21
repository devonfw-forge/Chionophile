use crate::AppState;
use actix_web::{Error, web};
use async_trait::async_trait;
use crate::api::common::errors::save_error::SaveError;
use crate::core::accesscodemanagement::logic::api::accesscode_cto::AccessCodeCto;
use crate::core::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::core::accesscodemanagement::rest::api::accesscode_post_data::AccessCodePostData;

#[async_trait]
pub trait UcManageAccessCode {
    async fn save_accesscode(
        app_state: web::Data<AppState>,
        accesscode_post_data: AccessCodePostData
    ) -> Result<AccessCodeEto, SaveError>;

    async fn cache_accesscode(
        app_state: web::Data<AppState>,
        accesscode: AccessCodeEto
    ) -> Result<AccessCodeEto, SaveError>;

    async fn cache_accesscode_cto(
        app_state: web::Data<AppState>,
        accesscode: AccessCodeCto
    ) -> Result<AccessCodeCto, SaveError>;

    async fn delete_accesscode(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<Option<i64>, Error>;
}
