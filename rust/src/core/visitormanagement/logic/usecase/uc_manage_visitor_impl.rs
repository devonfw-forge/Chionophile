use actix_web::{Error, web};
use async_trait::async_trait;
use validator::{Validate};
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::api::visitormanagement::logic::api::errors::save_error::SaveError;
use crate::api::visitormanagement::logic::usecase::uc_manage_visitor::UcManageVisitor;
use crate::AppState;
use crate::core::visitormanagement::dataaccess::api::new_visitor::NewVisitor;
use crate::core::visitormanagement::dataaccess::api::repo::visitor_repository_impl::VisitorRepositoryImpl;
use crate::core::visitormanagement::logic::api::visitor_eto::VisitorEto;
use crate::core::visitormanagement::dataaccess::api::visitor::Visitor;

pub struct UcManageVisitorImpl;

#[async_trait]
impl UcManageVisitor for UcManageVisitorImpl {
    async fn save_visitor(
        app_state: web::Data<AppState>,
        visitor: VisitorEto
    ) -> Result<VisitorEto, SaveError> {
        let result: Result<Visitor, SaveError> = web::block(move || {
            let conn = app_state.pool.get()?;
            visitor.validate()?;
            let new_visitor: NewVisitor = NewVisitor::from(visitor);
            let res = VisitorRepositoryImpl::save(&new_visitor, &conn)?;

            Ok(res)
        }).await?;

        let visitor = result?;

        Ok(visitor.into())
    }

    async fn delete_visitor(
        app_state: web::Data<AppState>,
        visitor_id: i64
    ) -> Result<bool, Error> {
        let deleted = web::block(move || {
            let conn = app_state.pool.get()?;
            VisitorRepositoryImpl::delete_by_id(visitor_id, &conn)
        }).await?.map_err(actix_web::error::ErrorInternalServerError)?;

        Ok(deleted)
    }
}

