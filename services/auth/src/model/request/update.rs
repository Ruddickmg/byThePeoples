use crate::model;
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateRequest {
    pub auth: model::EmailRequest,
    pub credentials: model::CredentialsRequest,
}

impl From<web::Json<UpdateRequest>> for UpdateRequest {
    fn from(json: web::Json<UpdateRequest>) -> UpdateRequest {
        UpdateRequest {
            auth: model::EmailRequest::new(&json.auth.email, &json.auth.password),
            credentials: model::CredentialsRequest::new(
                &json.credentials.password,
                &json.credentials.name,
                &json.credentials.email,
            ),
        }
    }
}

#[cfg(test)]
mod update_request_test {}
