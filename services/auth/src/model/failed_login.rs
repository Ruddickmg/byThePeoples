use crate::configuration::ALLOWED_FAILED_LOGIN_ATTEMPTS;
use crate::model::CredentialId;

pub struct FailedLogin {
    user_id: CredentialId,
    updated_at: database::Timestamp,
    created_at: database::Timestamp,
    attempts: i32,
}

impl FailedLogin {
    pub fn exceeded_limit(&self) -> bool {
        self.attempts > ALLOWED_FAILED_LOGIN_ATTEMPTS
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
