use crate::domain::config::dbtypes_config::DbError;
use actix_web::error::BlockingError;
use diesel::r2d2;
use std::fmt::{Display, Formatter};
use validator::ValidationErrors;

#[derive(Debug)]
pub enum SaveError {
    ValidationErrors(ValidationErrors),
    InternalServerError,
}

impl From<ValidationErrors> for SaveError {
    fn from(errors: ValidationErrors) -> Self {
        SaveError::ValidationErrors(errors)
    }
}
impl From<DbError> for SaveError {
    fn from(_: DbError) -> Self {
        SaveError::InternalServerError
    }
}
impl From<r2d2::PoolError> for SaveError {
    fn from(_: r2d2::PoolError) -> Self {
        SaveError::InternalServerError
    }
}
impl From<BlockingError> for SaveError {
    fn from(_: BlockingError) -> Self {
        SaveError::InternalServerError
    }
}
impl From<actix_web::Error> for SaveError {
    fn from(_: actix_web::Error) -> Self {
        SaveError::InternalServerError
    }
}
impl Display for SaveError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
