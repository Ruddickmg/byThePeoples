use super::{email_address, strong_password, user_name};
use crate::model;

pub fn full_request() -> model::FullRequest {
    model::FullRequest {
        name: user_name(),
        email: email_address(),
        password: strong_password(),
    }
}

pub fn credentials_request() -> model::CredentialsRequest {
    model::CredentialsRequest {
        name: Some(user_name()),
        email: Some(email_address()),
        password: Some(strong_password()),
    }
}

pub fn email_request() -> model::EmailRequest {
    model::EmailRequest {
        email: email_address(),
        password: strong_password(),
    }
}

pub fn name_request() -> model::NameRequest {
    model::NameRequest {
        name: user_name(),
        password: strong_password(),
    }
}

pub fn update_credentials_request() -> model::UpdateCredentials {
    model::UpdateCredentials {
        auth: email_request(),
        credentials: credentials_request(),
    }
}
