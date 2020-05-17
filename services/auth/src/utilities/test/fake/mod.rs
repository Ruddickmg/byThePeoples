use super::mock::{MockCredentials, MockLoginHistory};
use crate::model;
use fake::{faker::internet::en as internet, Fake};

mod credentials;
mod failed_login;
mod request;

pub use credentials::*;
pub use failed_login::*;
pub use request::*;

const MAX_FAKE_PASSWORD_LENGTH: usize = 20;
const MIN_FAKE_PASSWORD_LENGTH: usize = 15;

type MockServiceState = model::ServiceState<
    model::DatabaseConnection,
    MockLoginHistory<model::DatabaseConnection>,
    MockCredentials<model::DatabaseConnection>,
>;

pub fn strong_password() -> String {
    internet::Password(MIN_FAKE_PASSWORD_LENGTH..MAX_FAKE_PASSWORD_LENGTH).fake()
}

pub fn email_address() -> String {
    internet::FreeEmail().fake()
}

pub fn user_name() -> String {
    internet::Username().fake()
}

pub fn password_hash() -> String {
    String::from("lksg92q834thq3o74h93q4tt92qo4hgasofhg")
}

pub fn numeric_id() -> model::credentials::CredentialId {
    1
}

pub fn service_state() -> MockServiceState {
    let mock_login_history = MockLoginHistory::<model::DatabaseConnection>::new();
    let mock_credentials = MockCredentials::<model::DatabaseConnection>::new();
    model::ServiceState::new(mock_login_history, mock_credentials)
}
