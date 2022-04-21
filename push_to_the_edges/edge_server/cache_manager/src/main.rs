use std::time::Duration;
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::api::cache_service::CacheService;
use crate::core::accesscodemanagement::accesscode_repository::AccessCodeRepository;
use crate::core::queuemanagement::queue_repository::QueueRepository;
use self::core::cache_service::cache_service_impl::CacheServiceImpl;
use crate::core::visitormanagement::visitor_repository::VisitorRepository;
use crate::general::config::app_config::{ApplicationConfiguration};
use crate::general::config::dbtypes_config::DbConn;

#[macro_use]
extern crate diesel;

mod api;
mod core;
mod general;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app_config: ApplicationConfiguration = ApplicationConfiguration::init();
    let manager: ConnectionManager<DbConn> = ConnectionManager::<DbConn>::new(app_config.database_url);
    let pool: Pool<ConnectionManager<DbConn>> = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create POOL");

    loop {
        let _accesscode_res = CacheServiceImpl::delete_cache::<AccessCodeRepository>(pool.clone());
        let _queue_res = CacheServiceImpl::delete_cache::<QueueRepository>(pool.clone());
        let _visitor_res = CacheServiceImpl::delete_cache::<VisitorRepository>(pool.clone());

        tokio::time::sleep(Duration::from_millis(60000)).await;
    }
}
