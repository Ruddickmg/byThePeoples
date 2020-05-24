use database::Timestamp;
use std::time::{SystemTime, Duration};
use crate::{
    configuration::PASSWORD_RESET_TIME_PERIOD,
    utilities::hash,
    Error,
};

pub mod query {
    pub const GET_REQUEST_BY_ID: &str = "SELECT id, user_id, reset_token, created_at FROM auth.password_rest WHERE id = $1";
    pub const CREATE_REQUEST: &str = "INSERT VALUES($1, $2, $3) INTO auth.password_reset(id, user_id, reset_token)";
}

pub struct PasswordReset {
    id: String,
    reset_token: String,
    user_id: u32,
    created_at: Timestamp,
}

impl PasswordReset {
    pub fn expired(&self) -> Result<bool, Error> {
        Ok(SystemTime::now().duration_since(self.created_at)? > Duration::from_secs(PASSWORD_RESET_TIME_PERIOD))
    }
    pub fn matches_token(&self, token: &str) -> Result<bool, Error> {
        hash::authenticate(token, &self.reset_token)
    }
}

impl From<database::Row> for PasswordReset {
    fn from(row: database::Row) -> PasswordReset {
        PasswordReset {
            id: row.get(0),
            user_id: row.get(2),
            reset_token: row.get(1),
            created_at: row.get(3),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::hash;
    use std::time::{SystemTime, Duration};
    use crate::configuration::PASSWORD_RESET_TIME_PERIOD;
    use std::ops::Sub;

    #[test]
    fn expired_returns_true_if_password_request_has_expired() {
        let key = hash::token();
        let id = hash::token();
        let record = PasswordReset {
            id,
            reset_token: hash::generate(&key).unwrap(),
            user_id: 1,
            created_at: SystemTime::now().sub(Duration::from_secs(PASSWORD_RESET_TIME_PERIOD + 1)),
        };
        assert!(record.expired().unwrap())
    }

    #[test]
    fn expired_returns_false_if_password_request_has_not_expired() {
        let key = hash::token();
        let id = hash::token();
        let record = PasswordReset {
            id,
            reset_token: hash::generate(&key).unwrap(),
            user_id: 1,
            created_at: SystemTime::now(),
        };
        assert!(!record.expired().unwrap())
    }

    #[test]
    fn matches_token_returns_true_if_the_hashed_access_key_is_valid() {
        let key = hash::token();
        let id = hash::token();
        let record = PasswordReset {
            id,
            reset_token: hash::generate(&key).unwrap(),
            user_id: 1,
            created_at: SystemTime::now(),
        };
        assert!(record.matches_token(&key).unwrap())
    }

    #[test]
    fn matches_token_returns_false_if_the_hashed_access_key_is_invalid() {
        let key = hash::token();
        let id = hash::token();
        let record = PasswordReset {
            id: id.clone(),
            reset_token: hash::generate(&key).unwrap(),
            user_id: 1,
            created_at: SystemTime::now(),
        };
        assert!(!record.matches_token(&id).unwrap())
    }
}