use diesel::r2d2;
use validator::ValidationErrors;
use crate::core::general::config::dbtypes_config::DbError;

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

