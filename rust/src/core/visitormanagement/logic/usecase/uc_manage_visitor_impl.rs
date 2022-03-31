use actix_web::{Error, web};
use actix_web::error::BlockingError;
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
        let result: Result<Visitor, BlockingError<SaveError>> = web::block(move || {
            let conn = app_state.pool.get()?;
            visitor.validate()?;
            let new_visitor: NewVisitor = NewVisitor::from(visitor);
            let res = VisitorRepositoryImpl::save(&new_visitor, &conn)?;

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

    async fn delete_visitor(
        app_state: web::Data<AppState>,
        visitor_id: i64
    ) -> Result<(), Error> {
        web::block(move || {
            let conn = app_state.pool.get()?;
            VisitorRepositoryImpl::delete_by_id(visitor_id, &conn)
        }).await?;

        Ok(())
    }
}

