use crate::repository;
use std::marker::PhantomData;

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

pub type AppServiceState = ServiceState<
    DatabaseConnection,
    repository::LoginHistoryRepository<DatabaseConnection>,
    repository::CredentialsRepository<DatabaseConnection>,
>;

#[derive(Clone)]
pub struct ServiceState<
    T: Database = DatabaseConnection,
    L: repository::LoginHistory<T> = repository::LoginHistoryRepository<DatabaseConnection>,
    C: repository::Credentials<T> = repository::CredentialsRepository<DatabaseConnection>,
> {
    pub login_history: L,
    pub credentials: C,
    phantom: PhantomData<T>,
}

impl<T: Database, L: repository::LoginHistory<T>, C: repository::Credentials<T>>
    ServiceState<T, L, C>
{
    pub fn new(login_history: L, credentials: C) -> ServiceState<T, L, C> {
        ServiceState {
            credentials,
            login_history,
            phantom: PhantomData,
        }
    }
}

pub fn initialize_state(db: &DatabaseConnection) -> AppServiceState {
    let credentials_repository = repository::CredentialsRepository::new(db.clone());
    let login_history_repository = repository::LoginHistoryRepository::new(db.clone());
    ServiceState::new(login_history_repository, credentials_repository)
}
