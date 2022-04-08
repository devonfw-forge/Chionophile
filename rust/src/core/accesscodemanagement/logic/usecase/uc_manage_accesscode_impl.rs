use actix_web::{Error, web};
use actix_web::web::Data;
use async_trait::async_trait;
use chrono::Utc;
use crate::api::accesscodemanagement::logic::usecase::uc_manage_accesscode::UcManageAccessCode;
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::AppState;
use crate::core::accesscodemanagement::dataaccess::api::new_accesscode::NewAccessCode;
use crate::core::accesscodemanagement::dataaccess::api::repo::accesscode_repository_impl::AccessCodeRepositoryImpl;
use crate::core::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::core::accesscodemanagement::rest::api::accesscode_post_data::AccessCodePostData;

pub struct UcManageAccessCodeImpl;

#[async_trait]
impl UcManageAccessCode for UcManageAccessCodeImpl {
    async fn save_accesscode(
        app_state: Data<AppState>,
        accesscode_post_data: AccessCodePostData
    ) -> Result<AccessCodeEto, Error> {

        let access_code = web::block(move || {
            let conn = app_state.pool.get()?;

            let mut access_code_entity = NewAccessCode {
                modification_counter: 1,
                creation_time: None,
                start_time: None,
                end_time: None,
                queue_id: accesscode_post_data.queue_id.clone(),
                visitor_id: accesscode_post_data.visitor_id.clone()
            };

            access_code_entity.creation_time = Some(Utc::now().naive_utc());
            AccessCodeRepositoryImpl::save(&access_code_entity, &conn)

        }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(access_code.into())
    }

    async fn delete_accesscode(
        app_state: Data<AppState>,
        id: i64
    ) -> Result<(), Error> {
        web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::delete_by_id(id, &conn)
        }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(())
    }
}

