use crate::repository;

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
    repository::LoginHistoryRepository<DatabaseConnection>,
    repository::CredentialsRepository<DatabaseConnection>,
>;

#[derive(Clone)]
pub struct ServiceState<
    L: repository::LoginHistory,
    C: repository::Credentials,
> {
    pub login_history: L,
    pub credentials: C,
}

impl<L: repository::LoginHistory, C: repository::Credentials> ServiceState<L, C> {
    pub fn new(login_history: L, credentials: C) -> ServiceState<L, C> {
        ServiceState {
            credentials,
            login_history,
        }
    }
}

pub fn initialize_state(db: &DatabaseConnection) -> AppServiceState {
    let credentials_repository = repository::CredentialsRepository::new(db.clone());
    let login_history_repository = repository::LoginHistoryRepository::new(db.clone());
    ServiceState::new(login_history_repository, credentials_repository)
}
