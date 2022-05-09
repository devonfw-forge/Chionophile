use std::process::Command;


pub struct CacheScheduler;

impl CacheScheduler {
    pub fn init(database_url: &String) {
        let program = Command::new("cache_manager")
            .env("DATABASE_URL", database_url)
            .spawn();

        if program.is_err() {
            println!("Failed to initialize cache manager");
            println!("{}", program.err().unwrap());
        }
    }
}