use crate::repository;

pub mod credentials;
// mod failed_login;
mod request;

pub use credentials::*;
pub use database::Client;
pub use database::Database;
pub use database::DatabaseClient;
pub use database::DatabaseConnection;
// pub use failed_login::*;
pub use request::*;
use serde::export::PhantomData;

pub type AppServiceState<'a: 'b, 'b> = ServiceState<
    'a,
    // repository::LoginHistoryRepository<DatabaseConnection>,
    repository::CredentialsRepository<'a, 'b, DatabaseConnection>,
>;

struct Phantom;

#[derive(Clone)]
pub struct ServiceState<
    'a,
    C: repository::Credentials<'a>,
> {
    pub credentials: C,
    phantom: PhantomData<&'a Phantom>,
}

impl<'a, C: repository::Credentials<'a>> ServiceState<'a, C> {
    pub fn new(credentials: C) -> ServiceState<'a, C> {
        ServiceState {
            credentials,
            phantom: PhantomData,
        }
    }
}

pub fn initialize_state<'a: 'b, 'b>(db: &DatabaseConnection) -> AppServiceState<'a, 'b> {
    let credentials_repository = repository::CredentialsRepository::new(db.clone());
    // let login_history_repository = repository::LoginHistoryRepository::new(db.clone());
    ServiceState::new(credentials_repository)
}
