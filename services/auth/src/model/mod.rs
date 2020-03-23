use crate::{configuration::database as config, Error};
use std::{env, sync};

mod auth_request;
mod credential_request;
pub mod credentials;

pub type AuthRequest = auth_request::AuthRequest;
pub type Credentials = credentials::Credentials;
pub type CredentialRequest = credential_request::CredentialRequest;
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
