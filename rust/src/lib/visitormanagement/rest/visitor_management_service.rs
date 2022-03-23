use actix_web::{web, Error, HttpResponse};
use crate::lib::general::config::db_config::DbPool;
use crate::lib::visitormanagement::logic::api::visitor_eto::VisitorEto;
use crate::lib::visitormanagement::logic::visitor_management;
use crate::lib::visitormanagement::logic::api::visitor_search_criteria::VisitorSearchCriteria;


pub async fn find_visitors(
    pool: web::Data<DbPool>,
    visitor_filters: web::Json<VisitorSearchCriteria>
) -> Result<HttpResponse, Error> {
    let search_results =
        visitor_management
        ::find_visitors(pool, visitor_filters.into_inner())
            .await.map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(search_results))
}

pub async fn get_visitor(
    pool: web::Data<DbPool>,
    visitor_uid: web::Path<i64>
) -> Result<HttpResponse, Error> {

    let uid = visitor_uid.into_inner();
    let visitor_uid = uid.clone();
    let visitor =
        visitor_management
        ::find_visitor(pool, visitor_uid)
            .await.map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(visitor) = visitor {
        Ok(HttpResponse::Ok().json(visitor))
    } else {
        let res = HttpResponse::NotFound()
            .body(format!("No visitor found with uid: {}", uid));
        Ok(res)
    }

}

pub async fn save_visitor(
    pool: web::Data<DbPool>,
    form: web::Json<VisitorEto>
) -> Result<HttpResponse, Error> {

    let visitor = visitor_management::save_visitor(pool, form.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(visitor))
}

pub async fn delete_visitor(
    pool: web::Data<DbPool>,
    visitor_id: web::Path<i64>
) -> Result<HttpResponse, Error> {

    visitor_management::delete_visitor(pool, visitor_id.into_inner())
        .await
        .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().finish())
}
