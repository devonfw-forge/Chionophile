use actix_web::error::BlockingError;
use diesel::r2d2;
use validator::{ValidationErrors};
use crate::core::general::config::dbtypes_config::DbError;

/*
    Sometimes in a usecase method we have to manage multiple errors, such as database connection
    errors, field validation errors, etc. In a Result in Rust, we can only indicate one type of
    error, so we need to create an enum like this to be able to manage it correctly.
    This enum is made to manage the possible errors in the save_user method in uc_manage_user.
*/

#[derive(Debug)]
pub enum SaveError {
    ValidationErrors(ValidationErrors),
    InternalServerError
}

impl From<ValidationErrors> for SaveError {
    fn from(errors: ValidationErrors) -> Self {
        SaveError::ValidationErrors(errors)
    }
}
impl From<DbError> for SaveError {
    fn from (_: DbError) -> Self {
        SaveError::InternalServerError
    }
}
impl From<r2d2::PoolError> for SaveError {
    fn from (_: r2d2::PoolError) -> Self {
        SaveError::InternalServerError
    }
}
impl From<BlockingError> for SaveError {
    fn from (_: BlockingError) -> Self {
        SaveError::InternalServerError
    }
}

