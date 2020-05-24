use crate::{
    configuration::ACCOUNT_LOCK_DURATION_IN_SECONDS,
    utilities::hash,
    Error,
};
use std::time::{SystemTime, Duration};

pub type CredentialId = i32;

pub mod query {
    pub const NAME: &'static str = "SELECT id, email, name, hash, created_at, updated_at, deleted_at, locked_at FROM auth.credentials WHERE name = $1";
    pub const EMAIL: &str = "SELECT id, email, name, hash, created_at, updated_at, deleted_at, locked_at FROM auth.credentials WHERE email = $1";
    pub const SAVE: &str = "INSERT INTO auth.credentials(name, email, hash) VALUES ($1, $2, $3) RETURNING id, email, name, hash, created_at, updated_at, deleted_at, locked_at";
    pub const DELETED_AT: &str =
        "SELECT deleted_at FROM auth.credentials WHERE name = $1 OR email = $2";
    pub const UPDATE: &str = "UPDATE auth.credentials SET name = $1, hash = $2, email = $3, updated_at = CURRENT_TIMESTAMP, deleted_at = null WHERE id = $4 RETURNING id, email, name, hash, created_at, updated_at, deleted_at, locked_at";
    pub const DELETE_BY_EMAIL: &str =
        "UPDATE auth.credentials SET deleted_at = CURRENT_TIMESTAMP WHERE email = $1";
    pub const SUSPEND: &str =
        "UPDATE auth.credentials SET locked_at = CURRENT_TIMESTAMP WHERE id = $1";
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Credentials {
    pub id: CredentialId,
    pub email: String,
    pub name: String,
    pub hash: String,
    pub created_at: database::TimeStamp,
    pub updated_at: database::TimeStamp,
    pub deleted_at: Option<database::TimeStamp>,
    pub locked_at: Option<database::Timestamp>,
}

impl Credentials {
    pub fn suspended(&self) -> Result<bool, Error> {
        Ok(self.locked_at.map_or(false, | suspension_start | {
            let suspension_duration = Duration::from_secs(ACCOUNT_LOCK_DURATION_IN_SECONDS);
            SystemTime::now()
                .duration_since(suspension_start)
                .map(| start_time | start_time < suspension_duration)
                .unwrap_or(false)
        }))
    }
    pub fn password_matches(&self, password: &str) -> Result<bool, Error> {
        hash::authenticate(password, &self.hash)
    }
}

impl From<database::Row> for Credentials {
    fn from(row: database::Row) -> Credentials {
        Credentials {
            id: row.get(0),
            email: row.get(1),
            name: row.get(2),
            hash: row.get(3),
            created_at: row.get(4),
            updated_at: row.get(5),
            deleted_at: row.get(6),
            locked_at: row.get(7),
        }
    }
}

pub struct DeletedAt {
    pub deleted_at: Option<database::TimeStamp>,
}

impl From<database::Row> for DeletedAt {
    fn from(row: database::Row) -> Self {
        DeletedAt {
            deleted_at: row.get(0),
        }
    }
}

pub struct AffectedRows {
    pub count: i32,
}

impl From<database::Row> for AffectedRows {
    fn from(row: database::Row) -> Self {
        AffectedRows { count: row.get(0) }
    }
}

#[cfg(test)]
mod credentials_model_test {
    use crate::configuration::ACCOUNT_LOCK_DURATION_IN_SECONDS;
    use crate::utilities::test::fake;
    use actix_rt;
    use std::ops::Sub;
    use std::time::{Duration, SystemTime};

    #[actix_rt::test]
    async fn suspended_returns_true_if_the_time_since_suspended_is_less_than_the_timout_period() {
        let mut credentials = fake::credentials();
        credentials.locked_at = Some(SystemTime::now());
        assert_eq!(credentials.suspended().unwrap(), true);
    }

    #[actix_rt::test]
    async fn suspended_returns_false_if_the_time_since_suspended_is_longer_than_the_timout_period()
    {
        let mut credentials = fake::credentials();
        let time_longer_than_lock_duration =
            SystemTime::now().sub(Duration::from_secs(ACCOUNT_LOCK_DURATION_IN_SECONDS + 1));
        credentials.locked_at = Some(time_longer_than_lock_duration);
        assert_eq!(credentials.suspended().unwrap(), false);
    }

    #[actix_rt::test]
    async fn suspended_returns_false_if_the_account_was_never_suspended() {
        let credentials = fake::credentials();
        assert_eq!(credentials.suspended().unwrap(), false);
    }
}
