use crate::{configuration::ACCOUNT_LOCK_DURATION_IN_SECONDS, Error};
use std::time::Duration;

pub type CredentialId = i32;

pub mod query {
    pub const NAME: &'static str = "SELECT id, email, name, hash, created_at, updated_at, deleted_at, locked_at FROM auth.credentials WHERE name = $1";
    pub const EMAIL: &str = "SELECT id, email, name, hash, created_at, updated_at, deleted_at, locked_at FROM auth.credentials WHERE email = $1";
    pub const SAVE: &str = "INSERT INTO auth.credentials(name, email, hash) VALUES ($1, $2, $3)";
    pub const DELETED_AT: &str =
        "SELECT deleted_at FROM auth.credentials WHERE name = $1 OR email = $2";
    pub const UPDATE: &str = "UPDATE auth.credentials SET name = $1, hash = $2, email = $3, updated_at = CURRENT_TIMESTAMP, deleted_at = null WHERE id = $4 RETURNING id, email, name, hash, created_at, updated_at, deleted_at, locked_at";
    pub const DELETE_BY_EMAIL: &str =
        "UPDATE auth.credentials SET deleted_at = CURRENT_TIMESTAMP WHERE email = $1";
    pub const SUSPEND: &str =
        "UPDATE auth.credentials SET locked_at = CURRENT_TIMESTAMP WHERE id = $1";
}

#[derive(Debug, Clone)]
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
        if let Some(suspension_start) = self.locked_at {
            let now = database::TimeStamp::now();
            Ok(now.duration_since(suspension_start)?
                < Duration::from_secs(ACCOUNT_LOCK_DURATION_IN_SECONDS))
        } else {
            Ok(false)
        }
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
