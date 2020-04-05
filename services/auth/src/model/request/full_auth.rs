use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FullRequest {
    pub email: String,
    pub name: String,
    pub password: String,
}

impl FullRequest {
    pub fn new(name: &str, email: &str, password: &str) -> FullRequest {
        FullRequest {
            name: String::from(name),
            email: String::from(email),
            password: String::from(password),
        }
    }
}

impl From<web::Json<FullRequest>> for FullRequest {
    fn from(json: web::Json<FullRequest>) -> FullRequest {
        FullRequest {
            email: String::from(&json.email),
            name: String::from(&json.name),
            password: String::from(&json.password),
        }
    }
}
