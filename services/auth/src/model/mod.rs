use crate::{repository, Error};

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

pub type AppServiceState =
    ServiceState<DatabaseConnection, repository::CredentialsRepository<DatabaseConnection>>;

#[derive(Clone)]
pub struct ServiceState<T: Database, C: repository::Credentials<T>> {
    pub login_history: repository::LoginHistory<T>,
    pub credentials: C,
}

impl<T: Database, C: repository::Credentials<T>> ServiceState<T, C> {
    pub fn new(db: T, credentials: C) -> ServiceState<T, C> {
        ServiceState {
            credentials,
            login_history: repository::LoginHistory::new(db.clone()),
        }
    }
}

pub async fn initialize_state(db: &DatabaseConnection) -> Result<AppServiceState, Error> {
    let credentials_repository = repository::CredentialsRepository::new(db.clone());
    Ok(ServiceState::new(db.clone(), credentials_repository))
}
