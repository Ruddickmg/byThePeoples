use crate::{repository, Error};
use std::env;

pub mod credentials;
mod failed_login;
mod request;

pub use credentials::*;
pub use database::Client;
pub use database::Database;
pub use database::DatabaseClient;
pub use database::DatabaseConnection;
pub use failed_login::*;
pub use request::*;

#[derive(Clone)]
pub struct ServiceState<T: Database> {
    db: T,
    pub login_history: repository::LoginHistory<T>,
    pub credentials: repository::Credentials<T>,
}

impl<T: Database> ServiceState<T> {
    pub async fn new(db: T) -> Result<ServiceState<T>, Error> {
        Ok(ServiceState {
            db: db.clone(),
            credentials: repository::Credentials::new(db.clone()),
            login_history: repository::LoginHistory::new(db.clone()),
        })
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
