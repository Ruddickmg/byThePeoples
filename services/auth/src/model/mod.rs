use crate::Error;
use std::{env, sync};

mod auth_request;
pub mod credentials;

pub type AuthRequest = auth_request::AuthRequest;
pub type Database = database::Database;

pub const TEST_DATABASE_CONFIG: database::Configuration = database::Configuration {
    database: "postgres",
    password: "password",
    user: "postgres",
    host: "127.0.0.3",
    port: "8080",
};

pub struct ServiceState {
    pub db: sync::Mutex<Database>,
}

impl ServiceState {
    pub async fn new() -> Result<ServiceState, Error> {
        let db = database::ConnectionPool::new(TEST_DATABASE_CONFIG).await?;
        Ok(ServiceState {
            db: sync::Mutex::new(db),
        })
    }
    pub async fn initialize(self) -> Result<ServiceState, database::Error> {
        if environment::in_development() {
            let path_to_migrations = format!(
                "{}/src/sql/migrations",
                env::current_dir().unwrap().to_str().unwrap()
            );
            self.db.lock().unwrap().migrate(&path_to_migrations).await?;
            print!("Migration Successful.\n");
        }
        Ok(self)
    }
}
