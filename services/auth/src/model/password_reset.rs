use database::Timestamp;
use std::time::{SystemTime, Duration};
use serde::{Serialize, Deserialize};
use crate::{
    configuration::PASSWORD_RESET_TIME_PERIOD,
    utilities::hash,
    Result,
};

pub mod query {
    pub const GET_REQUEST_BY_ID: &str = "SELECT id, user_id, reset_token, name, email, created_at FROM auth.password_reset WHERE id = $1";
    pub const CREATE_REQUEST: &str = "INSERT INTO auth.password_reset(id, user_id, reset_token, name, email) VALUES($1, $2, $3, $4, $5) RETURNING id, user_id, reset_token, name, email, created_at";
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct PasswordResetRequest {
    pub id: String,
    pub user_id: i32,
    pub reset_token: String,
    pub name: String,
    pub email: String,
    pub created_at: Timestamp,
}

impl PasswordResetRequest {
    pub fn expired(&self) -> Result<bool> {
        Ok(SystemTime::now().duration_since(self.created_at)? > Duration::from_secs(PASSWORD_RESET_TIME_PERIOD))
    }
    pub fn matches_token(&self, token: &str) -> Result<bool> {
        hash::authenticate(token, &self.reset_token)
    }
}

impl From<database::Row> for PasswordResetRequest {
    fn from(row: database::Row) -> PasswordResetRequest {
        PasswordResetRequest {
            id: row.get(0),
            user_id: row.get(1),
            reset_token: row.get(2),
            name: row.get(3),
            email: row.get(4),
            created_at: row.get(5),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utilities::{hash, test::fake};
    use std::time::{SystemTime, Duration};
    use crate::configuration::PASSWORD_RESET_TIME_PERIOD;
    use std::ops::Sub;

    #[test]
    fn expired_returns_true_if_password_request_has_expired() {
        let key = hash::token();
        let id = hash::token();
        let mut record = fake::password_reset_request();
        record.id = id;
        record.reset_token = hash::generate(&key).unwrap();
        record.created_at = SystemTime::now().sub(Duration::from_secs(PASSWORD_RESET_TIME_PERIOD + 1));
        assert!(record.expired().unwrap())
    }

    #[test]
    fn expired_returns_false_if_password_request_has_not_expired() {
        let key = hash::token();
        let id = hash::token();
        let mut record = fake::password_reset_request();
        record.id = id;
        record.reset_token = hash::generate(&key).unwrap();
        assert!(!record.expired().unwrap())
    }

    #[test]
    fn matches_token_returns_true_if_the_hashed_access_key_is_valid() {
        let key = hash::token();
        let id = hash::token();
        let mut record = fake::password_reset_request();
        record.id = id;
        record.reset_token = hash::generate(&key).unwrap();
        assert!(record.matches_token(&key).unwrap())
    }

    #[test]
    fn matches_token_returns_false_if_the_hashed_access_key_is_invalid() {
        let id = hash::token();
        let record = fake::password_reset_request();
        assert!(!record.matches_token(&id).unwrap())
    }
}