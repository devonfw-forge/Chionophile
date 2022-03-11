use actix_web::{web, Error, HttpResponse};
use chrono::{Utc};
use uuid::Uuid;
use crate::lib::accesscodemanagement::logic::accesscode_management;
use crate::lib::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::lib::accesscodemanagement::rest::api::accesscode_post_data::AccessCodePostData;
use crate::lib::general::config::db_config::DbPool;

pub async fn get_accesscode_cto(
    pool: web::Data<DbPool>,
    accesscode_uid: web::Path<Uuid>
) -> Result<HttpResponse, Error> {

    let uid = accesscode_uid.into_inner();
    let accesscode_uid = uid.clone();
    let access_code = accesscode_management::find_accesscode_cto(pool, uid)
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(access_code) = access_code {
        Ok(HttpResponse::Ok().json(access_code))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No access_code found with uid: {}", accesscode_uid));
        Ok(res)
    }
}

pub async fn find_accesscode_ctos(
    pool: web::Data<DbPool>,
    criteria: web::Json<AccessCodeSearchCriteria>
) -> Result<HttpResponse, Error> {
    let search_results = accesscode_management::
    find_accesscode_ctos(pool, criteria.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(search_results))
}

pub async fn save_accesscode(
    pool: web::Data<DbPool>,
    accesscode_post_data: web::Json<AccessCodePostData>
) -> Result<HttpResponse, Error> {

    let accesscode_eto = accesscode_management::
    save_accesscode(pool, accesscode_post_data.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(accesscode_eto))
}

pub async fn delete_accesscode(
    pool: web::Data<DbPool>,
    access_code_uid: web::Path<Uuid>
) -> Result<HttpResponse, Error> {

    accesscode_management::delete_accesscode(pool, access_code_uid.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}

pub async fn find_accesscode_etos(
    pool: web::Data<DbPool>,
    criteria: web::Json<AccessCodeSearchCriteria>
) -> Result<HttpResponse, Error> {
    let search_results = accesscode_management::
    find_accesscode_etos(pool, criteria.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(search_results))
}


