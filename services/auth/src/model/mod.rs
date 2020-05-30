use crate::repository;

pub mod credentials;
mod failed_login;
pub mod password_reset;
mod request;
mod response;

pub use credentials::*;
pub use database::Client;
pub use database::Database;
pub use database::DatabaseClient;
pub use database::DatabaseConnection;
pub use failed_login::*;
pub use response::*;
pub use request::*;
pub use password_reset::*;

pub type AppServiceState = ServiceState<
    repository::LoginHistoryRepository<DatabaseConnection>,
    repository::CredentialsRepository<DatabaseConnection>,
    repository::PasswordReset<DatabaseConnection>,
>;

#[derive(Clone)]
pub struct ServiceState<
    L: repository::LoginHistory,
    C: repository::Credentials,
    R: repository::PasswordResetRequest,
> {
    pub login_history: L,
    pub credentials: C,
    pub reset_request: R,
}

impl<L: repository::LoginHistory, C: repository::Credentials, R: repository::PasswordResetRequest> ServiceState<L, C, R> {
    pub fn new(login_history: L, credentials: C, reset_request: R) -> ServiceState<L, C, R> {
        ServiceState {
            credentials,
            login_history,
            reset_request,
        }
    }
}

pub fn initialize_state(db: &DatabaseConnection) -> AppServiceState {
    let credentials_repository = repository::CredentialsRepository::new(db.clone());
    let login_history_repository = repository::LoginHistoryRepository::new(db.clone());
    let reset_request = repository::PasswordReset::new(db.clone());
    ServiceState::new(login_history_repository, credentials_repository, reset_request)
}
