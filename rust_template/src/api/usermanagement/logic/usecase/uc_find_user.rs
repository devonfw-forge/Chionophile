use actix_web::{Error, web};
use async_trait::async_trait;
use crate::AppState;
use crate::core::general::search::search_result::SearchResult;
use crate::core::usermanagement::logic::api::user_eto::UserEto;
use crate::core::usermanagement::logic::api::user_search_criteria::UserSearchCriteria;

#[async_trait]
pub trait UcFindUser {
    async fn find_user(
        app_state: web::Data<AppState>,
        id: i64
    ) -> Result<Option<UserEto>, Error>;

    async fn find_users(
        app_state: web::Data<AppState>,
        criteria: UserSearchCriteria
    ) -> Result<SearchResult<UserEto>, Error>;
}