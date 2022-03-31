use actix_web::{Error, web};
use actix_web::error::BlockingError;
use async_trait::async_trait;
use validator::{Validate};
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::api::usermanagement::logic::api::errors::save_errors::SaveError;
use crate::api::usermanagement::logic::usecase::uc_manage_user::UcManageUser;
use crate::AppState;
use crate::core::general::security::hashser::hash_password;
use crate::core::usermanagement::dataaccess::api::new_user::NewUser;
use crate::core::usermanagement::dataaccess::api::repo::user_repository_impl::UserRepositoryImpl;
use crate::core::usermanagement::logic::api::user_eto::UserEto;
use crate::core::usermanagement::dataaccess::api::user::User;

pub struct UcManageUserImpl;

#[async_trait]
impl UcManageUser for UcManageUserImpl {
    async fn save_user(
        app_state: web::Data<AppState>,
        user: UserEto
    ) -> Result<UserEto, SaveError> {
        let result: Result<User, BlockingError<SaveError>> = web::block(move || {
            let conn = app_state.pool.get()?;
            user.validate()?;
            let mut new_user: NewUser = NewUser::from(user);
            let hashed_password = hash_password(&new_user.password.unwrap());
            new_user.password = Some(hashed_password);
            let res = UserRepositoryImpl::save(&new_user, &conn)?;

            Ok(res)
        }).await;

        if let Err(result) = result {
            match result {
                BlockingError::Error(e) => {
                    Err(e)
                }
                BlockingError::Canceled => {
                    Err(SaveError::InternalServerError)
                }
            }
        } else {
            Ok(result.unwrap().into())
        }


    }

    async fn delete_user(
        app_state: web::Data<AppState>,
        user_id: i64
    ) -> Result<(), Error> {
        web::block(move || {
            let conn = app_state.pool.get()?;
            UserRepositoryImpl::delete_by_id(user_id, &conn)
        }).await?;

        Ok(())
    }
}

