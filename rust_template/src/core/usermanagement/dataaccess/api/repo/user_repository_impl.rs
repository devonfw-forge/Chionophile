use diesel::dsl::IntoBoxed;
use diesel::prelude::*;
use crate::api::common::dataaccess::api::entity::Entity;
use crate::api::common::dataaccess::api::repo::repository::Repository;
use crate::api::usermanagement::dataaccess::user_repository::UserRepository;
use crate::DbConn;
use crate::core::general::config::dbtypes_config::{DbError, DbType};
use crate::core::usermanagement::dataaccess::api::user::User;
use crate::core::usermanagement::logic::api::user_search_criteria::UserSearchCriteria;
use crate::core::general::database::schema::users;
use crate::core::general::security::hashser::hash_password;
use crate::core::usermanagement::dataaccess::api::new_user::NewUser;

pub struct UserRepositoryImpl;

impl UserRepository<i64, User, NewUser, UserSearchCriteria, users::table> for UserRepositoryImpl {}

impl Repository<i64, User, NewUser, UserSearchCriteria, users::table> for UserRepositoryImpl {
    fn find_by_id(
        user_id: i64,
        conn: &DbConn
    ) -> Result<Option<User>, DbError> {
        use crate::core::general::database::schema::users::dsl::*;

        let user: Option<User> = users
            .filter(id.eq(user_id))
            .first::<User>(conn)
            .optional()?;

        Ok(user)
    }

    fn find_by_criteria(
        criteria: UserSearchCriteria,
        conn: &DbConn
    ) -> Result<Vec<User>, DbError> {

        let mut query: IntoBoxed<users::table, DbType> = users::table.into_boxed::<DbType>();

        if let Some(username) = criteria.username {
            query = query.filter(users::username.eq(username));
        }
        if let Some(name) = criteria.name {
            query = query.filter(users::name.eq(name));
        }
        if let Some(phone_number) = criteria.phone_number {
            query = query.filter(users::phone_number.eq(phone_number));
        }
        if let Some(password) = criteria.password {
            let hash  = hash_password(&password);
            query = query.filter(users::password.eq(hash));
        }
        if let Some(accepted_commercial) = criteria.accepted_commercial {
            query = query.filter(users::accepted_commercial.eq(accepted_commercial));
        }
        if let Some(accepted_terms) = criteria.accepted_terms {
            query = query.filter(users::accepted_terms.eq(accepted_terms));
        }

        let query_results: Vec<User> = query.load(conn)?;


        Ok(query_results)
    }

    fn save(
        new_user: &NewUser,
        conn: &DbConn
    ) -> Result<User, DbError> {
        use crate::core::general::database::schema::users::dsl::*;

        let user_id = diesel::insert_into(users)
            .values(new_user)
            .returning(id)
            .get_result(conn)?;

        Ok(User::from_insert(user_id, new_user.clone()))
    }

    fn delete_by_id(
        user_id: i64,
        conn: &DbConn
    ) -> Result<i64, DbError> {
        use crate::core::general::database::schema::users::dsl::*;

        diesel::delete(users)
            .filter(id.eq(user_id))
            .execute(conn)?;

        Ok(user_id)
    }
}

