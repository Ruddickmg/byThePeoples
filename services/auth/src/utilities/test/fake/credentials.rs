use super::{email_address, numeric_id, user_name};
use crate::{model, utilities::test::fake::password_hash};
use std::time::SystemTime;

pub fn credentials() -> model::Credentials {
    model::Credentials {
        id: numeric_id(),
        email: email_address(),
        name: user_name(),
        hash: password_hash(),
        created_at: SystemTime::now(),
        updated_at: SystemTime::now(),
        deleted_at: None,
        locked_at: None,
    }
}
