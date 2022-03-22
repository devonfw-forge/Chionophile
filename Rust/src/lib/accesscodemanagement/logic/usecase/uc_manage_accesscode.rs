use actix_web::{Error, web};
use chrono::Utc;
use futures::executor::block_on;
use crate::lib::accesscodemanagement::dataaccess::api::new_access_code::NewAccessCode;
use crate::lib::accesscodemanagement::dataaccess::api::repo::accesscode_repository;
use crate::lib::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::lib::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::lib::accesscodemanagement::rest::api::accesscode_post_data::AccessCodePostData;
use crate::lib::general::config::db_config::DbPool;
use crate::lib::general::pageable::Pageable;
use crate::lib::queuemanagement::logic::queue_management;

pub async fn save_accesscode(
    pool: web::Data<DbPool>,
    accesscode_post_data: AccessCodePostData
) -> Result<AccessCodeEto, Error> {

 let access_code: AccessCodeEto = web::block(move || {
        let conn = pool.get()?;

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
                pageSize: 1000,
                pageNumber: 0,
                sort: None
            }
        };

        let queue_access_codes: Vec<AccessCodeEto> =
            accesscode_repository::find_etos(search_criteria, &conn)?;

        if queue_access_codes.is_empty() {
            access_code_entity.ticket_number = Some("Q000".to_string());
        } else {
            let last = queue_access_codes.last().unwrap().clone();
            access_code_entity.ticket_number = Some(generate_ticket_code(last));
        }

        access_code_entity.creation_time = Some(Utc::now().naive_utc());
        let queue_id = access_code_entity.queue_id.clone();
        block_on(queue_management::increase_queue_customer(pool, queue_id));
        accesscode_repository::save(&access_code_entity, &conn)

    }).await?;

    Ok(access_code)

}

pub async fn delete_accesscode(
    pool: web::Data<DbPool>,
    id: i64
) -> Result<(), Error> {

    web::block(move || {
        let conn = pool.clone().get()?;
        let accesscode_option = accesscode_repository::find_by_id(id.clone(), &conn)?;
        let accesscode = accesscode_option.unwrap();

        let queue_id = accesscode.queue_id;
        block_on(queue_management::decrease_queue_customer(pool, queue_id));
        accesscode_repository::delete(id, &conn)
    }).await?;

    Ok(())
}

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
