use diesel::{r2d2, PgConnection};
use diesel::r2d2::ConnectionManager;

/*
    Diesel requires all queries to specify the database type. It is not convenient to have
    DB specific types spread across all the codebase and sometimes it can be too verbose.
    This module provides an easy way to change the types when migrating the database.
*/
pub type DbConn = PgConnection;
pub type DbPool = r2d2::Pool<ConnectionManager<DbConn>>;
pub type DbType = diesel::pg::Pg;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;