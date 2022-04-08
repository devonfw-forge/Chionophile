use crate::AppState;
use actix_web::{Error, web};
use async_trait::async_trait;
use crate::api::visitormanagement::logic::api::errors::save_error::SaveError;
use crate::core::visitormanagement::logic::api::visitor_eto::VisitorEto;

#[async_trait]
pub trait UcManageVisitor {
    async fn save_visitor(
        app_state: web::Data<AppState>,
        visitor: VisitorEto
    ) -> Result<VisitorEto, SaveError>;

    async fn delete_visitor(
        app_state: web::Data<AppState>,
        visitor_id: i64
    ) -> Result<bool, Error>;
}
