pub mod database;
pub mod hash;
pub mod jwt;

pub const ACCOUNT_LOCK_DURATION_IN_MINUTES: u64 = 30;
pub const ALLOWED_FAILED_LOGIN_ATTEMPTS: i16 = 50;
