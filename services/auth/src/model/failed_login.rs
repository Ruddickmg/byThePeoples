use crate::{
    configuration::{ACCOUNT_LOCK_DURATION_IN_SECONDS, ALLOWED_FAILED_LOGIN_ATTEMPTS},
    model::CredentialId,
    Error,
};
use std::time::Duration;

#[derive(Clone, Debug)]
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

#[cfg(test)]
mod failed_login_model_test {
    use crate::configuration::{ACCOUNT_LOCK_DURATION_IN_SECONDS, ALLOWED_FAILED_LOGIN_ATTEMPTS};
    use crate::utilities::test::fake;
    use actix_rt;
    use std::ops::Sub;
    use std::time::{Duration, SystemTime};

    #[actix_rt::test]
    async fn exceeded_limit_returns_true_if_the_amount_of_login_attempts_exceeds_the_limit() {
        let mut failed_login = fake::failed_login();
        failed_login.attempts = ALLOWED_FAILED_LOGIN_ATTEMPTS + 1;
        assert_eq!(failed_login.exceeded_limit(), true);
    }

    #[actix_rt::test]
    async fn exceeded_limit_returns_false_if_the_amount_of_login_attempts_is_less_than_the_limit() {
        let mut failed_login = fake::failed_login();
        failed_login.attempts = 0;
        assert_eq!(failed_login.exceeded_limit(), false);
    }

    #[actix_rt::test]
    async fn expired_returns_true_if_the_time_since_suspension_is_greater_than_the_timeout_period()
    {
        let mut failed_login = fake::failed_login();
        failed_login.created_at =
            SystemTime::now().sub(Duration::from_secs(ACCOUNT_LOCK_DURATION_IN_SECONDS + 1));
        assert_eq!(failed_login.expired().unwrap(), true);
    }

    #[actix_rt::test]
    async fn expired_returns_false_if_the_time_since_suspension_is_less_than_the_timeout_period() {
        let failed_login = fake::failed_login();
        assert_eq!(failed_login.expired().unwrap(), false);
    }
}
