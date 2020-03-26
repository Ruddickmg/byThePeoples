use crate::{configuration::database as config, Error};
use std::{env, sync};

mod authentication;
pub mod credentials;

pub use authentication::Request as AuthRequest;
pub use credentials::Credentials;
pub use credentials::Request as CredentialRequest;

pub type Database = database::Database;

pub struct ServiceState {
    pub db: sync::Mutex<Database>,
}

impl ServiceState {
    pub async fn new() -> Result<ServiceState, Error> {
        let db = database::ConnectionPool::new(config::TEST_DATABASE_CONFIG).await?;
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
