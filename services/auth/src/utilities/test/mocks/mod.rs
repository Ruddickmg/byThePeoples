use crate::model;

mod repository;

pub use repository::*;

pub fn service_state() -> model::ServiceState<
    model::DatabaseConnection,
    MockLoginHistory<model::DatabaseConnection>,
    MockCredentials<model::DatabaseConnection>,
> {
    let mock_login_history = MockLoginHistory::<model::DatabaseConnection>::new();
    let mock_credentials = MockCredentials::<model::DatabaseConnection>::new();
    model::ServiceState::new(mock_login_history, mock_credentials)
}
