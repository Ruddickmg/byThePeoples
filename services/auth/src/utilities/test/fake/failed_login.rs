use super::numeric_id;
use crate::model;
use std::time::SystemTime;

pub fn failed_login() -> model::FailedLogin {
    model::FailedLogin {
        user_id: numeric_id(),
        updated_at: SystemTime::now(),
        created_at: SystemTime::now(),
        attempts: 0,
    }
}
