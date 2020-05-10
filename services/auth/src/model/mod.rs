use crate::Error;
use std::env;

pub mod credentials;
mod failed_login;
mod request;

pub use credentials::*;
pub use database::ConnectionPool as DatabaseConnection;
pub use database::Database;
pub use failed_login::*;
pub use request::*;

pub struct ServiceState<T: Database> {
    pub db: T,
}

impl<T: Database> ServiceState<T> {
    pub async fn new(db: T) -> Result<ServiceState<T>, Error> {
        Ok(ServiceState { db })
    }
    pub async fn initialize(self) -> Result<ServiceState<T>, database::Error> {
        if environment::in_development() {
            let path_to_migrations = format!(
                "{}/src/sql/migrations",
                env::current_dir().unwrap().to_str().unwrap()
            );
            self.db.migrate(&path_to_migrations).await?;
            println!("Migration Successful.\n");
        }
        Ok(self)
    }
}
