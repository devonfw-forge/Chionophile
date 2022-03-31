use actix_web::{Error, web};
use async_trait::async_trait;
use crate::api::usermanagement::logic::api::errors::save_errors::SaveError;
use crate::AppState;
use crate::core::usermanagement::logic::api::user_eto::UserEto;

#[async_trait]
pub trait UcManageUser {
    async fn save_user(
        app_state: web::Data<AppState>,
        user: UserEto
    ) -> Result<UserEto, SaveError>;

    async fn delete_user(
        app_state: web::Data<AppState>,
        user_id: i64
    ) -> Result<(), Error>;
}