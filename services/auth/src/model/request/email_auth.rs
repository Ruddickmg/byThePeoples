use crate::{model, Error};
use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EmailRequest {
    pub email: String,
    pub password: String,
}

impl EmailRequest {
    pub fn new(email: &str, password: &str) -> EmailRequest {
        EmailRequest {
            email: String::from(email),
            password: String::from(password),
        }
    }
}

impl From<model::FullRequest> for EmailRequest {
    fn from(credentials: model::FullRequest) -> EmailRequest {
        EmailRequest::new(&credentials.email, &credentials.password)
    }
}

impl From<web::Json<EmailRequest>> for EmailRequest {
    fn from(json: web::Json<EmailRequest>) -> EmailRequest {
        EmailRequest {
            email: String::from(&json.email),
            password: String::from(&json.password),
        }
    }
}
