use crate::AppState;
use actix_web::{Error, web};
use async_trait::async_trait;
use crate::core::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::core::accesscodemanagement::rest::api::accesscode_post_data::AccessCodePostData;

#[async_trait]
pub trait UcManageAccessCode {
    async fn save_accesscode(
        app_state: web::Data<AppState>,
        accesscode_post_data: AccessCodePostData
    ) -> Result<AccessCodeEto, Error>;

    async fn delete_accesscode(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<(), Error>;
}
