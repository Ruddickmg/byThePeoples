use crate::model;
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateCredentials {
    pub auth: model::EmailRequest,
    pub credentials: model::CredentialsRequest,
}

impl From<web::Json<UpdateCredentials>> for UpdateCredentials {
    fn from(input: web::Json<UpdateCredentials>) -> UpdateCredentials {
        UpdateCredentials::new(&input.auth, &input.credentials)
    }
}

impl UpdateCredentials {
    pub fn new(
        auth: &model::EmailRequest,
        credentials: &model::CredentialsRequest,
    ) -> UpdateCredentials {
        UpdateCredentials {
            auth: auth.clone(),
            credentials: credentials.clone(),
        }
    }
}

#[cfg(test)]
mod update_request_test {}
