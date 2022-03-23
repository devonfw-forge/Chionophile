use actix_web::{Error, web};
use crate::lib::general::config::dbtypes_config::DbPool;
use crate::lib::general::search::search_result::SearchResult;
use crate::lib::usermanagement::logic::api::user_search_criteria::UserSearchCriteria;
use crate::lib::usermanagement::logic::api::user_eto::UserEto;
use crate::lib::usermanagement::logic::usecase::{uc_find_user, uc_manage_user};

pub async fn find_user(
    pool: web::Data<DbPool>,
    id: i64,
) -> Result<Option<UserEto>, Error> {
    uc_find_user::find_user(pool, id).await
}

pub async fn find_users(
    pool: web::Data<DbPool>,
    criteria: UserSearchCriteria
) -> Result<SearchResult<UserEto>, Error> {
    uc_find_user::find_users(pool, criteria).await
}

pub async fn save_user(
    pool: web::Data<DbPool>,
    user: UserEto
) -> Result<UserEto, Error> {
    uc_manage_user::save_user(pool, user).await
}

pub async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: i64
) -> Result<bool, Error> {
    uc_manage_user::delete_user(pool, user_id).await
}
