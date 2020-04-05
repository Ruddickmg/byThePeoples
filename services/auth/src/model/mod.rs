use crate::{configuration::database as config, Error};
use std::env;

pub(crate) mod credentials;
mod request;

pub use credentials::*;
pub use database::Database;
pub use request::*;

pub struct ServiceState {
    pub db: Database,
}

impl ServiceState {
    pub async fn new() -> Result<ServiceState, Error> {
        let db = database::Database::new(config::TEST_DATABASE_CONFIG).await?;
        Ok(ServiceState { db })
    }
    pub async fn initialize(self) -> Result<ServiceState, database::Error> {
        if environment::in_development() {
            let path_to_migrations = format!(
                "{}/src/sql/migrations",
                env::current_dir().unwrap().to_str().unwrap()
            );
            self.db.migrate(&path_to_migrations).await?;
            print!("Migration Successful.\n");
        }
        Ok(self)
    }
}
