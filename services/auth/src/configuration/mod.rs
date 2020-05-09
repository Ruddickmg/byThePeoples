use crate::constants::{HOURS_IN_A_DAY, MINUTES_IN_AN_HOUR, SECONDS_IN_A_MINUTE};

pub mod database;
pub mod hash;
pub mod jwt;

pub const ACCOUNT_LOCK_DURATION_IN_SECONDS: u64 =
    SECONDS_IN_A_MINUTE * MINUTES_IN_AN_HOUR * HOURS_IN_A_DAY;
pub const ALLOWED_FAILED_LOGIN_ATTEMPTS: i16 = 50;
