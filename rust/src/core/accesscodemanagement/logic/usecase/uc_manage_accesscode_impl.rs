use actix_web::{Error, web};
use actix_web::web::Data;
use async_trait::async_trait;
use chrono::Utc;
use crate::api::accesscodemanagement::logic::usecase::uc_manage_accesscode::UcManageAccessCode;
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::AppState;
use crate::core::accesscodemanagement::dataaccess::api::accesscode::AccessCode;
use crate::core::accesscodemanagement::dataaccess::api::new_accesscode::NewAccessCode;
use crate::core::accesscodemanagement::dataaccess::api::repo::accesscode_repository_impl::AccessCodeRepositoryImpl;
use crate::core::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::core::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::core::accesscodemanagement::rest::api::accesscode_post_data::AccessCodePostData;
use crate::core::general::search::pageable::Pageable;

pub struct UcManageAccessCodeImpl;

#[async_trait]
impl UcManageAccessCode for UcManageAccessCodeImpl {
    async fn save_accesscode(
        app_state: Data<AppState>,
        accesscode_post_data: AccessCodePostData
    ) -> Result<AccessCodeEto, Error> {

        let access_code: AccessCode = web::block(move || {
            let conn = app_state.pool.get()?;

            let mut access_code_entity = NewAccessCode {
                modification_counter: 1,
                ticket_number: None,
                creation_time: None,
                start_time: None,
                end_time: None,
                queue_id: accesscode_post_data.queue_id.clone(),
                visitor_id: accesscode_post_data.visitor_id.clone()
            };

            let search_criteria = AccessCodeSearchCriteria {
                ticket_number: None,
                creation_time: None,
                start_time: None,
                end_time: None,
                visitor_id: None,
                queue_id: Some(accesscode_post_data.queue_id.clone()),
                pageable: Pageable {
                    page_size: 0,
                    page_number: 0,
                    sort: None
                }
            };

            let queue_access_codes: Vec<AccessCode> =
                AccessCodeRepositoryImpl::find_by_criteria(search_criteria, &conn)?;

            if queue_access_codes.is_empty() {
                access_code_entity.ticket_number = Some("Q000".to_string());
            } else {
                let last: AccessCodeEto = queue_access_codes.last().unwrap().clone().into();
                access_code_entity.ticket_number = Some(Self::generate_ticket_code(&last));
            }

            access_code_entity.creation_time = Some(Utc::now().naive_utc());
            AccessCodeRepositoryImpl::save(&access_code_entity, &conn)

        }).await?;

        Ok(access_code.into())
    }

    async fn delete_accesscode(
        app_state: Data<AppState>,
        id: i64
    ) -> Result<(), Error> {
        web::block(move || {
            let conn = app_state.pool.get()?;
            AccessCodeRepositoryImpl::delete_by_id(id, &conn)
        }).await?;

        Ok(())
    }
}

impl UcManageAccessCodeImpl {
    fn generate_ticket_code(last_access_code: &AccessCodeEto) -> String {
        let last_ticket_number = last_access_code.ticket_number.clone().unwrap();
        let ticket_digit = last_ticket_number[1..].to_string().parse::<i32>().unwrap();
        let new_digit = ticket_digit + 1;
        return if new_digit == 1000 {
            String::from("Q000")
        } else {
            let mut digit_string = new_digit.to_string();
            while digit_string.chars().count() < 3 {
                digit_string.insert(0, '0');
            }
            digit_string.insert(0, 'Q');
            digit_string
        }
    }
}

