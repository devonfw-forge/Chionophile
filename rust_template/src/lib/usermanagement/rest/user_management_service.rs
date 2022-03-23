use actix_web::{HttpResponse, web, Error};
use crate::lib::general::config::dbtypes_config::DbPool;
use crate::lib::usermanagement::logic::api::user_search_criteria::UserSearchCriteria;
use crate::lib::usermanagement::logic::api::user_eto::UserEto;
use crate::lib::usermanagement::logic::user_management;

/*
    The service will be responsible for contacting the logic layer and then sending back
    the response to the FrontEnd. Each method can have the database pool as an argument because 
    we previously added it as app_data when creating the Application.
*/

pub async fn search_user(
    pool: web::Data<DbPool>,
    search_criteria_box: web::Json<UserSearchCriteria>
) -> Result<HttpResponse, Error> {
    let search_results =
        user_management
        ::find_users(pool, search_criteria_box.into_inner())
            .await.map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(search_results))
}

pub async fn get_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i64>
) -> Result<HttpResponse, Error> {
    let uid = user_id.into_inner();
    let user_uid = uid.clone();
    let user =
        user_management
        ::find_user(pool, user_uid)
            .await.map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No user found with uid: {}", uid));
        Ok(res)
    }

}
pub async fn save_user(
    pool: web::Data<DbPool>,
    user: web::Path<UserEto>
) -> Result<HttpResponse, Error> {

    let user = user_management::save_user(pool, user.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
}

pub async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: web::Path<i64>
) -> Result<HttpResponse, Error> {
    user_management::delete_user(pool, user_id.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}
