use crate::{
    configuration::{ACCOUNT_LOCK_DURATION_IN_SECONDS, ALLOWED_FAILED_LOGIN_ATTEMPTS},
    model::CredentialId,
    Error,
};
use std::time::Duration;

pub struct FailedLogin {
    pub user_id: CredentialId,
    pub updated_at: database::Timestamp,
    pub created_at: database::Timestamp,
    pub attempts: database::SmallInt,
}

impl FailedLogin {
    pub fn exceeded_limit(&self) -> bool {
        self.attempts > ALLOWED_FAILED_LOGIN_ATTEMPTS
    }
    pub fn expired(&self) -> Result<bool, Error> {
        let now = database::TimeStamp::now();
        Ok(now.duration_since(self.created_at)?
            > Duration::from_secs(ACCOUNT_LOCK_DURATION_IN_SECONDS))
    }
}

impl From<database::Row> for FailedLogin {
    fn from(row: database::Row) -> FailedLogin {
        FailedLogin {
            user_id: row.get(0),
            attempts: row.get(1),
            created_at: row.get(2),
            updated_at: row.get(3),
        }
    }
}

impl From<&database::Row> for FailedLogin {
    fn from(row: &database::Row) -> FailedLogin {
        FailedLogin {
            user_id: row.get(0),
            attempts: row.get(1),
            created_at: row.get(2),
            updated_at: row.get(3),
        }
    }
}
