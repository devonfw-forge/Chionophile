use diesel::prelude::*;
use crate::DbConn;
use crate::lib::general::config::dbtypes_config::{DbError, DbType};
use crate::lib::usermanagement::dataaccess::api::user::User;
use crate::lib::usermanagement::logic::api::user_search_criteria::UserSearchCriteria;
use crate::lib::general::database::schema::users;
use crate::lib::usermanagement::dataaccess::api::new_user::NewUser;

pub fn find_by_id(
    user_id: i64,
    conn: &DbConn
) -> Result<Option<User>, DbError> {
    use crate::lib::general::database::schema::users::dsl::*;

    let user: Option<User> = users
        .filter(id.eq(user_id))
        .first::<User>(conn)
        .optional()?;

    Ok(user)
}

pub fn find_by_criteria(
    criteria: UserSearchCriteria,
    conn: &DbConn
) -> Result<Vec<User>, DbError> {

    let mut query = users::table.into_boxed::<DbType>();

    if let Some(username) = criteria.username {
        //add_string_filter(query, users::username, &criteria.username_option, &username);
        query = query.filter(users::username.eq(username));
    }
    if let Some(name) = criteria.name {
        query = query.filter(users::name.eq(name));
    }
    if let Some(phone_number) = criteria.phone_number {
        query = query.filter(users::phone_number.eq(phone_number));
    }
    if let Some(password) = criteria.password {
        query = query.filter(users::password.eq(password));
    }
    if let Some(accepted_commercial) = criteria.accepted_commercial {
        query = query.filter(users::accepted_commercial.eq(accepted_commercial));
    }
    if let Some(accepted_terms) = criteria.accepted_terms {
        query = query.filter(users::accepted_terms.eq(accepted_terms));
    }

    let query_results: Vec<User> = query.load(conn)?;

    let paged_results = criteria.pageable.from(query_results);

    Ok(paged_results)
}

pub fn save(
    new_user: &NewUser,
    conn: &DbConn
) -> Result<User, DbError> {
    use crate::lib::general::database::schema::users::dsl::*;

    let user_id = diesel::insert_into(users)
        .values(new_user)
        .returning(id)
        .get_result(conn)?;

    Ok(User::from_insert(user_id, new_user.clone()))
}

pub fn delete_by_id(
    user_id: i64,
    conn: &DbConn
) -> Result<i64, DbError> {
    use crate::lib::general::database::schema::users::dsl::*;

    diesel::delete(users)
        .filter(id.eq(user_id))
        .execute(conn)?;

    Ok(user_id)
}