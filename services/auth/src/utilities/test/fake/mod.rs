use super::mock::{MockCredentials, MockLoginHistory, MockPasswordReset};
use crate::{model, utilities::hash};
use fake::{faker::internet::en as internet, Fake};

mod credentials;
mod failed_login;
mod request;

pub use credentials::*;
pub use failed_login::*;
pub use request::*;
use std::time::SystemTime;

const MAX_FAKE_PASSWORD_LENGTH: usize = 20;
const MIN_FAKE_PASSWORD_LENGTH: usize = 15;
const WEAK_PASSWORD: &str = "password";

type MockServiceState = model::ServiceState<
    MockLoginHistory<model::DatabaseConnection>,
    MockCredentials<model::DatabaseConnection>,
    MockPasswordReset<model::DatabaseConnection>,
>;

pub fn strong_password() -> String {
    internet::Password(MIN_FAKE_PASSWORD_LENGTH..MAX_FAKE_PASSWORD_LENGTH).fake()
}
pub fn weak_password() -> String {
    String::from(WEAK_PASSWORD)
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

pub fn password_reset_request() -> model::PasswordResetRequest {
    model::PasswordResetRequest {
        id: hash::token(),
        reset_token: hash::token(),
        user_id: 0,
        name: user_name(),
        email: email_address(),
        created_at: SystemTime::now(),
    }
}

pub fn password_reset_token() -> model::ResetToken {
    model::ResetToken::new(hash::token().as_ref(), hash::token().as_ref())
}

pub fn password_reset_data() -> model::ResetConfirmation {
    model::ResetConfirmation {
        id: hash::token(),
        reset_token: hash::token(),
        password: strong_password(),
    }
}

pub fn reset_request() -> model::ResetRequest {
    model::ResetRequest {
        email: email_address(),
    }
}

pub fn service_state() -> MockServiceState {
    let mock_login_history = MockLoginHistory::<model::DatabaseConnection>::new();
    let mock_credentials = MockCredentials::<model::DatabaseConnection>::new();
    let mock_password_reset = MockPasswordReset::<model::DatabaseConnection>::new();
    model::ServiceState::new(mock_login_history, mock_credentials, mock_password_reset)
}
