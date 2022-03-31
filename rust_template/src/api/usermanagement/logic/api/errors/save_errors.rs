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
    DbError(DbError),
    ConnectionError(r2d2::PoolError),
    InternalServerError
}

impl From<ValidationErrors> for SaveError {
    fn from(errors: ValidationErrors) -> Self {
        SaveError::ValidationErrors(errors)
    }
}
impl From<DbError> for SaveError {
    fn from (error: DbError) -> Self {
        SaveError::DbError(error)
    }
}
impl From<r2d2::PoolError> for SaveError {
    fn from (error: r2d2::PoolError) -> Self {
        SaveError::ConnectionError(error)
    }
}

