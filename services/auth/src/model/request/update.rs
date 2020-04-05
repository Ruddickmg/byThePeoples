use crate::model;
use actix_web::error::DispatchError::Upgrade;
use actix_web::web;
use serde::{Deserialize, Serialize};

pub enum UpdateRequest {
    Valid(UpdateCredentials),
    Invalid,
}

impl From<web::Json<OptionUpdateRequest>> for UpdateRequest {
    fn from(input: web::Json<OptionUpdateRequest>) -> Self {
        match &input.auth {
            Some(auth) => {
                if auth.email.is_none() || auth.password.is_none() {
                    UpdateRequest::Invalid
                } else {
                    match &input.credentials {
                        Some(credentials) => UpdateRequest::Valid(UpdateCredentials::new(
                            &model::EmailRequest::new(
                                &auth.clone().email.unwrap(),
                                &auth.clone().password.unwrap(),
                            ),
                            &credentials,
                        )),
                        None => UpdateRequest::Invalid,
                    }
                }
            }
            None => UpdateRequest::Invalid,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OptionUpdateRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth: Option<model::OptionEmailRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credentials: Option<model::CredentialsRequest>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UpdateCredentials {
    pub auth: model::EmailRequest,
    pub credentials: model::CredentialsRequest,
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

impl From<web::Json<UpdateCredentials>> for UpdateCredentials {
    fn from(json: web::Json<UpdateCredentials>) -> UpdateCredentials {
        UpdateCredentials {
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
