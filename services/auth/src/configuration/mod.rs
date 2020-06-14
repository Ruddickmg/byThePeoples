use crate::constants::ONE_DAY;

pub use environment;

pub mod connection;
pub mod database;
pub mod hash;
pub mod jwt;

pub const ACCOUNT_LOCK_DURATION_IN_SECONDS: u64 = ONE_DAY;
pub const PASSWORD_RESET_TIME_PERIOD: u64 = ONE_DAY;
pub const ALLOWED_FAILED_LOGIN_ATTEMPTS: i16 = 50;
