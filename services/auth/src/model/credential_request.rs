use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CredentialRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

impl From<web::Json<CredentialRequest>> for CredentialRequest {
    fn from(json: web::Json<CredentialRequest>) -> CredentialRequest {
        CredentialRequest {
            email: String::from(&json.email),
            name: String::from(&json.name),
            password: String::from(&json.password),
        }
    }
}
