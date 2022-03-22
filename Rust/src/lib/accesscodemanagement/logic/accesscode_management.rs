use actix_web::{Error, web};
use crate::lib::accesscodemanagement::logic::api::accesscode_cto::AccessCodeCto;
use crate::lib::accesscodemanagement::logic::api::accesscode_eto::AccessCodeEto;
use crate::lib::accesscodemanagement::logic::api::accesscode_search_criteria::AccessCodeSearchCriteria;
use crate::lib::accesscodemanagement::logic::usecase::{uc_find_accesscode, uc_manage_accesscode};
use crate::lib::accesscodemanagement::rest::api::accesscode_post_data::AccessCodePostData;
use crate::lib::general::config::db_config::DbPool;
use crate::lib::general::search_result::SearchResult;

pub async fn find_accesscode_cto(
    pool: web::Data<DbPool>,
    id: i64,
) -> Result<Option<AccessCodeCto>, Error> {
    uc_find_accesscode::find_accesscode_cto(pool, id).await
}

pub async fn find_accesscode_ctos(
    pool: web::Data<DbPool>,
    criteria: AccessCodeSearchCriteria,
) -> Result<SearchResult<AccessCodeCto>, Error> {
    uc_find_accesscode::find_accesscode_ctos(pool, criteria).await
}

pub async fn save_accesscode(
    pool: web::Data<DbPool>,
    accesscode: AccessCodePostData
) -> Result<AccessCodeEto, Error> {
    uc_manage_accesscode::save_accesscode(pool, accesscode).await
}


pub async fn delete_accesscode(
    pool: web::Data<DbPool>,
    id: i64
) -> Result<(), Error> {
    uc_manage_accesscode::delete_accesscode(pool, id).await
}

pub async fn find_accesscode_etos(
    pool: web::Data<DbPool>,
    criteria: AccessCodeSearchCriteria
) -> Result<SearchResult<AccessCodeEto>, Error> {
    uc_find_accesscode::find_accesscode_etos(pool, criteria).await
}





