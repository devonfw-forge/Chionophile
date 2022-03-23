use diesel::{r2d2, PgConnection};
use diesel::r2d2::ConnectionManager;

pub type DbConn = PgConnection;
pub type DbPool = r2d2::Pool<ConnectionManager<DbConn>>;
pub type DbType = diesel::pg::Pg;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;