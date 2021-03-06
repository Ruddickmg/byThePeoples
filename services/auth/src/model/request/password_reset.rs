use actix_web::web;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResetConfirmation {
    pub id: String,
    pub reset_token: String,
    pub password: String,
}

impl ResetConfirmation {
    pub fn new(id: &str, reset_token: &str, password: &str) -> ResetConfirmation {
        ResetConfirmation {
            id: String::from(id),
            reset_token: String::from(reset_token),
            password: String::from(password),
        }
    }
}

impl From<web::Json<ResetConfirmation>> for ResetConfirmation {
    fn from(json: web::Json<ResetConfirmation>) -> ResetConfirmation {
        ResetConfirmation {
            id: String::from(&json.id),
            reset_token: String::from(&json.reset_token),
            password: String::from(&json.password),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResetRequest {
    pub email: String,
}

impl ResetRequest {
    pub fn new(email: &str) -> ResetRequest {
        ResetRequest { email: String::from(email) }
    }
}

impl From<web::Json<ResetRequest>> for ResetRequest {
    fn from(json: web::Json<ResetRequest>) -> ResetRequest {
        ResetRequest {
            email: String::from(&json.email),
        }
    }
}
