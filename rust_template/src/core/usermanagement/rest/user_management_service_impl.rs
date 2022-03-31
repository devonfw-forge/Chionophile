use actix_web::{HttpResponse, web, Error};
use actix_web::web::{Data, Path};
use crate::core::usermanagement::logic::api::user_search_criteria::UserSearchCriteria;
use crate::core::usermanagement::logic::api::user_eto::UserEto;
use async_trait::async_trait;
use crate::api::common::rest::crud_rest_service::CRUDRestService;
use crate::api::usermanagement::logic::api::errors::save_errors::SaveError;
use crate::api::usermanagement::logic::usecase::uc_find_user::UcFindUser;
use crate::api::usermanagement::logic::usecase::uc_manage_user::UcManageUser;
use crate::api::usermanagement::rest::api::user_management_service::UserManagementService;
use crate::AppState;
use crate::core::usermanagement::logic::user_management_impl::UserManagementImpl;

/*
    The service will be responsible for contacting the logic layer and then sending back
    the response to the FrontEnd. Each method can have the database pool in the AppState as an
    argument because we previously added it as app_data when creating the Application.
*/

pub struct UserManagementServiceImpl;

impl UserManagementService for UserManagementServiceImpl {}

#[async_trait]
impl CRUDRestService<i64, UserEto, UserSearchCriteria, UserEto> for UserManagementServiceImpl {
    async fn search(
        app_state: Data<AppState>,
        criteria: web::Json<UserSearchCriteria>
    ) -> Result<HttpResponse, Error> {
        let search_results =
            UserManagementImpl
            ::find_users(app_state, criteria.into_inner())
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().json(search_results))
    }

    async fn get(
        app_state: Data<AppState>,
        id: Path<i64>
    ) -> Result<HttpResponse, Error> {
        let id = id.into_inner();
        let user_id = id.clone();
        let user =
            UserManagementImpl
            ::find_user(app_state, user_id)
                .await
                .map_err(actix_web::error::ErrorInternalServerError)?;

        if let Some(user) = user {
            Ok(HttpResponse::Ok().json(user))
        } else {
            let res = HttpResponse::NotFound()
                .body(format!("No user found with uid: {}", id));
            Ok(res)
        }
    }

    async fn save(
        app_state: Data<AppState>,
        user: web::Json<UserEto>
    ) -> Result<HttpResponse, Error> {
        println!("SAVE USER");
        let save_result: Result<UserEto, SaveError> =
            UserManagementImpl
            ::save_user(app_state, user.into_inner())
                .await;
        match save_result {
            Ok(user) => {
                Ok(HttpResponse::Ok().json(user))
            }
            Err(save_error) => {
                match save_error {
                    SaveError::ValidationErrors(validation_errors) => {
                        Ok(HttpResponse::BadRequest().json(validation_errors))
                    }
                    SaveError::DbError(_)
                    | SaveError::ConnectionError(_)
                    | SaveError::InternalServerError => {
                        Ok(HttpResponse::InternalServerError().finish())
                    }
                }
            }
        }

    }

    async fn delete(
        app_state: Data<AppState>,
        id: Path<i64>
    ) -> Result<HttpResponse, Error> {
        UserManagementImpl::
        delete_user(app_state, id.into_inner())
            .await
            .map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(HttpResponse::Ok().finish())
    }
}
