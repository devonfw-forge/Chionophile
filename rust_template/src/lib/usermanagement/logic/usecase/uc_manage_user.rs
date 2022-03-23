use actix_web::{Error, web};
use crate::lib::general::config::dbtypes_config::DbPool;
use crate::lib::usermanagement::dataaccess::api::new_user::NewUser;
use crate::lib::usermanagement::dataaccess::api::repo::user_repository;
use crate::lib::usermanagement::logic::api::user_eto::UserEto;

pub async fn save_user(
    pool: web::Data<DbPool>,
    user: UserEto
) -> Result<UserEto, Error> {
    let result = web::block(move || {
        let conn = pool.get()?;
        let new_user: NewUser = NewUser::from(user);

        user_repository::save(&new_user, &conn)
    }).await?;

    Ok(UserEto::from(result))
}

pub async fn delete_user(
    pool: web::Data<DbPool>,
    user_id: i64
) -> Result<bool, Error> {
    web::block(move || {
        let conn = pool.get()?;
        user_repository::delete_by_id(user_id, &conn)
    }).await?;

    Ok(true)
}
