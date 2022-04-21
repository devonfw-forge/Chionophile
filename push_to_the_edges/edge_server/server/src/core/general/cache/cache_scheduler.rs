use std::process::Command;


pub struct CacheScheduler;

impl CacheScheduler {
    pub fn init(database_url: &String) {
        let cache_program = if cfg!(windows) {
            "cache_manager.exe"
        } else {
            "./cache_manager"
        };
        let program = Command::new(cache_program)
            .env("DATABASE_URL", database_url)
            .spawn();

        if program.is_err() {
            println!("Failed to initialize cache manager");
            println!("{}", program.err().unwrap());
        }
    }
}