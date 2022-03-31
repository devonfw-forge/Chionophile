use actix_web::{Error, web};
use async_trait::async_trait;
use crate::api::usermanagement::logic::api::errors::save_errors::SaveError;
use crate::api::usermanagement::logic::usecase::uc_find_user::UcFindUser;
use crate::api::usermanagement::logic::usecase::uc_manage_user::UcManageUser;
use crate::api::usermanagement::logic::user_management::UserManagement;
use crate::AppState;
use crate::core::general::search::search_result::SearchResult;
use crate::core::usermanagement::logic::api::user_search_criteria::UserSearchCriteria;
use crate::core::usermanagement::logic::api::user_eto::UserEto;
use crate::core::usermanagement::logic::usecase::uc_find_user_impl::UcFindUserImpl;
use crate::core::usermanagement::logic::usecase::uc_manage_user_impl::UcManageUserImpl;

pub struct UserManagementImpl;

impl UserManagement for UserManagementImpl {}

#[async_trait]
impl UcManageUser for UserManagementImpl {
    async fn save_user(
        app_state: web::Data<AppState>,
        user: UserEto
    ) -> Result<UserEto, SaveError> {
        UcManageUserImpl::save_user(app_state, user).await
    }

    async fn delete_user(
        app_state: web::Data<AppState>,
        user_id: i64
    ) -> Result<(), Error> {
        UcManageUserImpl::delete_user(app_state, user_id).await
    }

}

#[async_trait]
impl UcFindUser for UserManagementImpl {
    async fn find_user(
        app_state: web::Data<AppState>,
        id: i64,
    ) -> Result<Option<UserEto>, Error> {
        UcFindUserImpl::find_user(app_state, id).await
    }

    async fn find_users(
        app_state: web::Data<AppState>,
        criteria: UserSearchCriteria
    ) -> Result<SearchResult<UserEto>, Error> {
        UcFindUserImpl::find_users(app_state, criteria).await
    }
}

